from django.db import models


# Create your models here.
class BaseModel(models.Model):
    objects = models.Manager()
    unique_field: str
    unique_relation_model: models.Model

    relation_id_dict = {}

    class Meta:
        abstract = True
        ordering = ["-pk"]

    @staticmethod
    def display_field_name() -> str:
        # Qualifies Model to be used in the DropdownView
        raise NotImplementedError

    @property
    def default_related_name(self) -> str:
        return self.__class__.__name__.lower()

    @staticmethod
    def reverse_related_field_model_map(model) -> dict[str, models.Model]:
        """
        Get fields from reverse relationships and their associated models.

        Returns
        -------
        dict
            Dict of field names and models.

        Examples
        --------
        >>> Messstelle.reverse_related_field_model_map()
        {'status': Status, 'position': Position, 'kabel': Kabel, 'sensor': Sensor}
        """
        related_model_map = {}
        for field in model._meta.get_fields():
            if field.is_relation and not isinstance(field, models.ForeignKey):
                related_model_map[field.name] = field.field.model
        return related_model_map

    @staticmethod
    def get_non_relation_fields(model) -> set:
        return {
            field.name
            for field in model._meta.fields
            if not field.is_relation and not field.primary_key
        }

    @staticmethod
    def get_foreign_key_field_map(model) -> dict[str, models.Model]:
        return {
            field.name: field.target_field.model
            for field in model._meta.fields
            if isinstance(field, models.ForeignKey)
        }

    @staticmethod
    def get_reversed_relation_path(path: str):
        # A method to reverse the relation path.
        relation_step_list = path.split("__")
        # Removes the id and the name of the last object.
        relation_step_list = relation_step_list[: len(relation_step_list) - 2]
        reversed_path = ""
        for step in reversed(relation_step_list):
            reversed_path += step + "__"

        reversed_path += "id"
        return reversed_path

    def get_relation_field_name_from_relation_path(self, relation_path):
        # Returns the name of the field from the relation_path
        return self.get_relation_field_name(relation_path.split("__")[0])

    def get_relation_field_name(self, default_related_name):
        # Returns the name of the field from the default_related_name
        for field in self._meta.fields:
            if (
                field.is_relation
                and field.related_model().default_related_name == default_related_name
            ):
                return field.name

    def get_relation_path(self, target_model):
        return self._generate_relation_path(self, target_model)

    def _generate_relation_path(self, instance, target_model):
        # A method to generate the relation path from two model objects. Returns None if there is no valid path to the other object.

        # TODO At the moment, the relation path is generatet only from the foreigen key fields.
        for field in instance._meta.fields:
            if field.is_relation:
                related_model = field.related_model()
                if (
                    related_model.default_related_name
                    == target_model.default_related_name
                ):
                    return f"{related_model.default_related_name}__id"
                else:
                    path = self._generate_relation_path(related_model, target_model)
                    if path is not None:
                        return f"{field.name}__{path}"

    def get(self, field_name, default):
        return getattr(self, field_name, default)
