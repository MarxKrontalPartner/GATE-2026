import copy

from django.db import DatabaseError
from rest_framework import serializers

from core.assetverwaltung.models.assetverwaltung_models import BaseModel
from core.assetverwaltung.serializers.base import BaseViewSerializer


class IncludeReverseRelationMixin(BaseViewSerializer):
    """
    This should be handled like an abstract class.
    """

    class Meta:
        model: BaseModel  # overwrite this
        included_reverse_relations: list[str]  # overwrite this

    def pop_model_data(self, d: dict) -> dict:
        model_data = {}
        for field in self.Meta.model._meta.fields:
            if not field.primary_key and field.name in d:
                model_data[field.name] = d.pop(field.name)
        return model_data

    def create(self, validated_data):
        model = self.Meta.model
        model_data = self.pop_model_data(validated_data)

        # substitutes relation id with relation object
        if self.many:
            for field in self.Meta.model._meta.fields:
                if field.is_relation:
                    relation_object = field.related_model.objects.filter(
                        pk=model_data.get(field.name, None)
                    ).first()
                    model_data.update(**{field.name: relation_object})

        try:
            # create instance
            instance = model.objects.create(**model_data)
            # create nested one-to-one models
            for revrel_name in self.Meta.included_reverse_relations:
                revrel_model = getattr(model, revrel_name).related.field.model
                related_model_data = validated_data.get(revrel_name, {})
                related_model_data[instance.default_related_name] = instance
                revrel_model.objects.create(**related_model_data)
        except DatabaseError as e:
            msg = (
                f"Unexpected error when trying to create nested {instance.default_related_name} objects. "
                f"The following exception was raised - {type(e)}: {e}"
            )
            raise Exception(msg) from e

        return instance

    def update(self, instance, validated_data):
        """
        Extends default update method to provide nested relation support.

        Data from nested relations is popped from validated_data.
        The popped data is passed to an update call on the respective related serializers.
        """
        try:
            # iterate over copy so keys can be popped from original
            for k, _v in copy.deepcopy(validated_data).items():
                if k in self.Meta.included_reverse_relations:
                    nested_instance = getattr(instance, k)
                    nested_data = validated_data.pop(k)
                    self.fields[k].update(nested_instance, nested_data)
            # update instance
            inst = super().update(instance, validated_data)
        except DatabaseError as e:
            msg = (
                f"Unexpected error when trying to update nested {instance.default_related_name} objects. "
                f"The following exception was raised - {type(e)}: {e}"
            )
            raise Exception(msg) from e

        return inst


class BulkUpdateListSerializerMixin(serializers.ListSerializer):
    def validate(self, attrs):
        data_ids = {i["id"] for i in attrs}  # all ids from the request
        instance_ids = set(
            self.instance.values_list("pk", flat=True)
        )  # all ids from the queryset
        # Check if all ids from the request are in the queryset
        if data_ids - instance_ids:
            raise serializers.ValidationError("Cannot update non-existing objects")
        return super().validate(attrs)

    def update(self, instance, validated_data):
        # Maps for id->instance and id->data item.
        instance_mapping = {obj.id: obj for obj in instance}
        data_mapping = {item["id"]: item for item in validated_data}

        # Perform creations and updates
        ret = []
        for instance_id, data in data_mapping.items():
            # Getting the instance from the mapping should never fail,
            # because the validation ensures that all ids are in the queryset.
            instance = instance_mapping[instance_id]
            ret.append(self.child.update(instance, data))
        return ret
