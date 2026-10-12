from django.db import IntegrityError, models, transaction
from django.forms import ValidationError
from rest_framework import serializers
from rest_framework.relations import PrimaryKeyRelatedField
from rest_framework.utils.serializer_helpers import ReturnDict


class BaseViewSerializer(serializers.ModelSerializer):
    """
    This serializer is a ModelSerializer rewritten to be a ListSerializer at the same time.
    This makes it easier to inherit and use the functionality in other serializers.
    Most of the functions were taken and copied from the inheritance and only adapted in the places
    where it has to handle multiple objects.
    """

    # To avoid that init creates a ListSerializer, because this Serializer will handle multiple updates or creates.
    def __new__(cls, *args, **kwargs):
        many = kwargs.pop("many", False)
        cls.many = many
        if hasattr(cls.Meta, "list_serializer_class"):
            raise Exception(
                "To use the BaseViewSerializer, you are not allowed to use a ListSerializer, because this serializer will handle multiple updates and creations. You can use these methods for that."
            )
        return super().__new__(cls, *args, **kwargs)

    def is_valid(self, *, raise_exception=False):
        if isinstance(self.initial_data, list):
            self._validated_data = []
            for object in self.initial_data:
                try:
                    self._validated_data.append(self.run_validation(object))
                except ValidationError as exc:
                    self._validated_data = {}
                    self._errors = exc.detail

            if not hasattr(self, "_errors"):
                self._errors = {}

            if self._errors and raise_exception:
                raise ValidationError(self.errors)

            self._validated_data = self.initial_data.copy()

            return not bool(self._errors)
        else:
            return super().is_valid(raise_exception=raise_exception)

    # Runs create or update with atomic transaction
    def save(self, **kwargs):
        assert hasattr(self, "_errors"), (
            "You must call `.is_valid()` before calling `.save()`."
        )

        assert not self.errors, (
            "You cannot call `.save()` on a serializer with invalid data."
        )

        # Guard against incorrect use of `serializer.save(commit=False)`
        assert "commit" not in kwargs, (
            "'commit' is not a valid keyword argument to the 'save()' method. "
            "If you need to access data before committing to the database then "
            "inspect 'serializer.validated_data' instead. "
            "You can also pass additional keyword arguments to 'save()' if you "
            "need to set extra attributes on the saved model instance. "
            "For example: 'serializer.save(owner=request.user)'.'"
        )

        assert not hasattr(self, "_data"), (
            "You cannot call `.save()` after accessing `serializer.data`."
            "If you need to access data before committing to the database then "
            "inspect 'serializer.validated_data' instead. "
        )

        with transaction.atomic():
            try:
                # List Serializer und Modelserializer zusammenführen
                if self.many:
                    if not isinstance(self.validated_data, list):
                        raise serializers.ValidationError(
                            "You cannot update only one object if you specify a update-list."
                        )
                    validated_data = [
                        {**attrs, **kwargs} for attrs in self.validated_data
                    ]

                    if self.instance is not None:
                        self.instance = []
                        for object in validated_data:
                            if "id" in object:
                                if self.Meta.model.objects.filter(
                                    pk=object["id"]
                                ).exists():
                                    self.instance.append(
                                        self.update(
                                            instance=self.Meta.model.objects.get(
                                                pk=object["id"]
                                            ),
                                            validated_data=object,
                                        )
                                    )
                                else:
                                    raise serializers.ValidationError(
                                        {
                                            "id": "Object with id "
                                            + str(object["id"])
                                            + " not found."
                                        }
                                    )
                            else:
                                raise serializers.ValidationError(
                                    {
                                        "id": "To update a list, you must set the id field for each object."
                                    }
                                )
                        assert self.instance is not None, (
                            "`update()` did not return an object instance."
                        )
                    else:
                        self.instance = []
                        for object in validated_data:
                            self.instance.append(self.create(object))
                        assert self.instance is not None, (
                            "`create()` did not return an object instance."
                        )
                else:
                    if isinstance(self.validated_data, list):
                        raise serializers.ValidationError(
                            "You cannot update a list if you specify pk"
                        )
                    validated_data = {**self.validated_data, **kwargs}

                    if self.instance is not None:
                        self.instance = self.update(self.instance, validated_data)
                        assert self.instance is not None, (
                            "`update()` did not return an object instance."
                        )
                    else:
                        self.instance = self.create(validated_data)
                        assert self.instance is not None, (
                            "`create()` did not return an object instance."
                        )

                return self.instance
            except IntegrityError as e:
                infos_field = e.args[0].split(".")[1]
                info_error = e.args[0].split(":")[0]
                raise serializers.ValidationError({infos_field: info_error}) from None

    def to_representation(self, instance):
        """
        This function is needed to support multiple objects in the serializer.
        """
        if isinstance(instance, models.query.QuerySet) or isinstance(instance, list):
            return [self.to_representation(item) for item in instance]
        else:
            return super().to_representation(instance)

    def create(self, validated_data):
        for field_name, field in self.fields.items():
            # If the field is a PrimaryKeyRelatedField, it will convert the pk to the object.
            if (
                isinstance(field, PrimaryKeyRelatedField)
                and field_name in validated_data
                and validated_data[field_name] is not None
                and isinstance(validated_data[field_name], int)
            ):
                model_class = field.get_queryset().model
                validated_data[field_name] = model_class.objects.get(
                    pk=validated_data[field_name]
                )
        return super().create(validated_data)

    def update(self, instance, validated_data):
        for field_name, field in self.fields.items():
            # If the field is a PrimaryKeyRelatedField, it will convert the pk to the object.
            if (
                isinstance(field, PrimaryKeyRelatedField)
                and field_name in validated_data
                and validated_data[field_name] is not None
                and isinstance(validated_data[field_name], int)
            ):
                model_class = field.get_queryset().model
                validated_data[field_name] = model_class.objects.get(
                    pk=validated_data[field_name]
                )
        return super().update(instance, validated_data)

    @property
    def data(self):
        """
        Needed to overwrite the data from ModelSerializer as this throws an error.
        """

        if (
            hasattr(self, "initial_data")
            and not hasattr(self, "_validated_data")
            and not hasattr(self, "instance")
        ):
            msg = (
                "When a serializer is passed a `data` keyword argument you "
                "must call `.is_valid()` before attempting to access the "
                "serialized `.data` representation.\n"
                "You should either call `.is_valid()` first, "
                "or access `.initial_data` instead."
            )
            raise AssertionError(msg)

        if not hasattr(self, "_data") or len(getattr(self, "_data", None)) == 0:
            if self.instance is not None and not getattr(self, "_errors", None):
                self._data = self.to_representation(self.instance)
            elif hasattr(self, "_validated_data") and not getattr(
                self, "_errors", None
            ):
                self._data = self.to_representation(self.validated_data)
            else:
                self._data = self.get_initial()
        """Is needed in the debug frontend of django!"""
        if isinstance(self._data, dict):
            return ReturnDict(self._data, serializer=self)
        return self._data
