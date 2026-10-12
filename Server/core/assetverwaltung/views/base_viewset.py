from django.http import QueryDict
from rest_framework import status, viewsets
from rest_framework.decorators import action
from rest_framework.response import Response

from core.assetverwaltung.models.assetverwaltung_models import BaseModel


class BaseViewSet(viewsets.ModelViewSet):
    """
    This BaseViewSet is a normal ModelViewSet, but with the functionality that get objects can be sorted by a query parameter
    and this viewset supports creating and update rqeuests with lists.
    This viewset automatically adds the path update-list and partial-update-list.
    """

    class Meta:
        query_filter_relation_model: BaseModel
        query_field_name: str
        order_field: str  # Optional
        model: BaseModel

    def get_queryset(self):
        # Adds the functionality that there can be a query parameter to sort with the relation.
        # If there is no model defined in Meta there are other options to load the queryset.
        if not hasattr(self.Meta, "model"):
            return super().get_queryset()

        model = self.Meta.model
        objects: QueryDict

        # Checks if the query_filter_relation_path and query_field_name is set and filters the objects if the queryparams is set.
        if hasattr(self.Meta, "query_filter_relation_model") and hasattr(
            self.Meta, "query_field_name"
        ):
            query_filter_relation_model = self.Meta.query_filter_relation_model
            query_field_name = self.Meta.query_field_name
            if self.Meta.query_field_name in self.request.query_params:
                relation_id = self.request.query_params.get(query_field_name, "")
                if (
                    not query_filter_relation_model.objects.filter(
                        pk=relation_id
                    ).count()
                    == 0
                ):
                    filter_relation_path = model().get_relation_path(
                        query_filter_relation_model()
                    )
                    objects = model.objects.filter(
                        **{filter_relation_path: relation_id}
                    )
                else:
                    return None
            else:
                objects = model.objects.all()
        else:
            objects = model.objects.all()

        # Checks if the order_field is set and orders the objects
        if hasattr(self.Meta, "order_field"):
            return objects.order_by(self.Meta.order_field)
        else:
            return objects

    def create(self, request, *args, **kwargs):
        # many parameter added
        serializer = self.get_serializer(
            data=request.data, many=isinstance(request.data, list)
        )
        serializer.is_valid(raise_exception=True)
        serializer.save()
        headers = self.get_success_headers(serializer.data)
        return Response(
            serializer.data, status=status.HTTP_201_CREATED, headers=headers
        )

    @action(detail=False, methods=["put"], url_path="update-list")
    def update_list(self, request, *args, **kwargs):
        partial = kwargs.pop("partial", False)
        serializer = self.get_serializer(
            instance=self.Meta.model.objects.all(),
            data=request.data,
            partial=partial,
            many=True,
        )
        serializer.is_valid(raise_exception=True)
        serializer.save()
        headers = self.get_success_headers(serializer.data)
        return Response(serializer.data, status=status.HTTP_200_OK, headers=headers)

    @action(detail=False, methods=["patch"], url_path="partial-update-list")
    def partial_update_list(self, request, *args, **kwargs):
        kwargs["partial"] = True
        return self.update_list(request, *args, **kwargs)
