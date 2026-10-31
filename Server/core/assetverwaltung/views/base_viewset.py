from django.db import transaction
from django.http import QueryDict
from rest_framework import exceptions, status, viewsets
from rest_framework.decorators import action
from rest_framework.response import Response

from core.assetverwaltung.models.assetverwaltung_models import BaseModel
from core.assetverwaltung.serializers.base import SyncSubmitSerializer


class BaseViewSet(viewsets.ModelViewSet):
    """
    This BaseViewSet is a normal ModelViewSet, but with the functionality that get objects can be sorted by a query parameter
    and this viewset supports creating and update rqeuests with lists.
    This viewset automatically adds the path update-list and partial-update-list.
    """
    
    """
    It also adds the path sync, which only works for models with an EDITABLE list and
    a version field.
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


    def _check_patch(self, instance, patch):
        server_data = self.get_serializer(instance).data

        changes, conflicts = {}, []
        for op in patch:
            field, base, new = op["field"], op.get("base"), op["new"]
            current = server_data[field]

            if current == base:
                changes[field] = new
            elif current != new:
                conflicts.append({"field": field, "server_value": current})

        return changes, conflicts

    def _build_update_list(self, submits, instances):
        """
        Turn the patch list into a plain partial update list.

        Returns the update list for the objects without a conflict and the
        conflict details for all others. Objects without changes are in neither.
        """
        update_data, conflict_details = [], []

        for submit in submits:
            instance = instances[submit["id"]]
            changes, conflicts = self._check_patch(
                instance, submit["patch"]
            )

            # One conflicting field marks the whole object as a conflict
            if conflicts:
                conflict_details.append(
                    {
                        "id": instance.pk,
                        "conflicts": conflicts,
                        "version": instance.version,
                    }
                )
            elif changes:
                # The list update has no perform_update, so the version is sent along
                update_data.append(
                    {"id": instance.pk, **changes, "version": instance.version + 1}
                )

        return update_data, conflict_details

    def perform_update(self, serializer):
        if self.action == "sync":
            serializer.save(version=serializer.instance.version + 1)
        else:
            super().perform_update(serializer)

    @action(detail=False, methods=["patch"])
    def sync(self, request, *args, **kwargs):
  
        model = getattr(self.Meta, "model", None)
        if not hasattr(model, "EDITABLE"):
            raise exceptions.MethodNotAllowed(
                request.method, detail="This model does not support sync."
            )

        many = isinstance(request.data, list)
        submit_serializer = SyncSubmitSerializer(
            data=request.data, many=many, context={"model": model}
        )
        submit_serializer.is_valid(raise_exception=True)
        validated_data = submit_serializer.validated_data
        submits = validated_data if many else [validated_data]
        ids = [submit["id"] for submit in submits]

        with transaction.atomic():
            # Lock the rows while the patch is checked and saved
            instances = self.get_queryset().select_for_update().in_bulk(ids)
            missing = [pk for pk in ids if pk not in instances]
            if missing:
                return Response(
                    {"status": "NOT_FOUND", "ids": missing},
                    status=status.HTTP_404_NOT_FOUND,
                )

            update_data, conflict_details = self._build_update_list(submits, instances)

            saved = []
            if len(update_data) == 1:
                # One object goes through partial_update, it reads the id from kwargs
                request._full_data = update_data[0]
                self.kwargs[self.lookup_field] = update_data[0]["id"]
                saved = [viewsets.ModelViewSet.partial_update(self, request).data]
            elif update_data:
                # Multiple objects go through partial_update_list
                request._full_data = update_data
                saved = BaseViewSet.partial_update_list(self, request).data

            return Response(
                {
                    "status": "CONFLICT" if conflict_details else "OK",
                    "saved": saved,
                    "conflicts": conflict_details,
                },
                status=status.HTTP_409_CONFLICT
                if conflict_details
                else status.HTTP_200_OK,
            )
