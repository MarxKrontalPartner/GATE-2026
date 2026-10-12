from rest_framework import status, viewsets
from rest_framework.response import Response


# ToDO write custom metadata_class to enable options requests on
# the bulk endpoint. Right now getting metadata for put and patch endpoints
# also expects a pk to be send, which is not the case for the bulk endpoint.
# https://www.django-rest-framework.org/api-guide/metadata/
class BulkUpdateViewSetMixin(viewsets.ModelViewSet):
    """
    Implements `bulk_update` and `partial_bulk_update` methods
    for bulk updates to non detail views.

    Because of the underlying implementation of serializers,
    an additional `bulk_update_serializer_class` is required for bulk updates.
    This `bulk_update_serializer_class` has to define it's own `update` method for bulk updates.
    """

    bulk_update_serializer_class = None

    def get_serializer_class(self):
        """
        Overwrite the default `get_serializer_class` method.

        Return either the `bulk_update_serializer_class` or the default `serializer_class`.
        Bulk update is only allowed on list views, so `self.detail` must be false.
        """
        if not self.detail and (
            self.action == "update_bulk" or self.action == "partial_update_bulk"
        ):
            assert self.bulk_update_serializer_class is not None, (
                f"'{self.__class__.__name__}' should either include a `bulk_update_serializer_class` attribute, "
                "or override the `get_serializer_class()` method."
            )

            return self.bulk_update_serializer_class
        else:
            return super().get_serializer_class()

    def update_bulk(self, request, *args, **kwargs):
        partial = kwargs.pop("partial", False)
        serializer = self.get_serializer(
            instance=self.get_queryset(), data=request.data, partial=partial, many=True
        )
        serializer.is_valid(raise_exception=True)
        self.perform_update(serializer)

        headers = self.get_success_headers(serializer.data)
        return Response(serializer.data, status=status.HTTP_200_OK, headers=headers)

    def partial_update_bulk(self, request, *args, **kwargs):
        kwargs["partial"] = True
        return self.update_bulk(request, *args, **kwargs)


class BulkPostViewSetMixin(viewsets.ModelViewSet):
    def create(self, request, *args, **kwargs):
        serializer = self.get_serializer(
            data=request.data, many=isinstance(request.data, list)
        )
        serializer.is_valid(raise_exception=True)
        self.perform_create(serializer)
        headers = self.get_success_headers(serializer.data)
        return Response(
            serializer.data, status=status.HTTP_201_CREATED, headers=headers
        )


class BulkDeleteViewSetMixin(viewsets.ModelViewSet):
    def delete_bulk(self, request, *args, **kwargs):
        """
        This actually expects to be triggered by a post request, not a delete request,
        as delete requests are not meant to have a body according to the rfc.
        """
        if not isinstance(request.data, list):
            return Response(status=status.HTTP_400_BAD_REQUEST)
        queryset = self.get_queryset()
        # delete instances for given pks. Non-existent pks are ignored.
        instances = queryset.filter(pk__in=request.data)
        count, objects = instances.delete()
        return Response({"count": count}, status=status.HTTP_200_OK)


class BauwerkQuerySetMixin:
    """Intended to be used on some sort of modelViewSet."""

    queryset_bauwerk_path = None  # overwrite this. used to reverse bauwerk relation

    def get_queryset(self):
        filters = {}
        # get instances for this bauwerk only
        if bauwerk_id := self.request.query_params.get("bauwerk_id", False):
            filters[self.queryset_bauwerk_path] = bauwerk_id
        model = (
            self.serializer_class.Meta.model
        )  # requires viewset to have a serializer_class
        return model.objects.filter(**filters)


class UnserializedListMixin(viewsets.ModelViewSet):
    def list(self, request, *args, **kwargs):
        queryset = self.get_queryset()
        meta_fields = self.serializer_class.Meta.model._meta.get_fields(
            include_parents=False
        )
        # probably the best way to simulate _meta.local_fields without using it, as it's an internal attribute
        fields = [
            field.name
            for field in meta_fields
            if not (field.is_relation and field.auto_created)
        ]
        return Response(queryset.values(*fields))
