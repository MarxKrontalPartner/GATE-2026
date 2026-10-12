import copy

from rest_framework.routers import DefaultRouter, Route, SimpleRouter


class CustomRouter(DefaultRouter):
    """
    Copies the routes of SimpleRouter and adds
    update_bulk, partial_update_bulk and delete_bulk actions to the list route.
    """

    routes = copy.deepcopy(SimpleRouter.routes)
    if routes[0].url == r"^{prefix}{trailing_slash}$":
        routes[0].mapping.update(
            {
                "put": "update_bulk",
                "patch": "partial_update_bulk",
            }
        )
    else:
        raise ValueError("Expected the first route to be the list route.")

    """
    bulk delete route

    Note this is using post instead of delete.
    Delete requests are not meant to have a body according to
    the rfc (https://www.rfc-editor.org/rfc/rfc9110.html#name-delete),
    nor are they well supported.
    """
    # insert in the beginning to ensure the delete-bulk path is not used up by some wildcard path
    routes.insert(
        1,
        Route(
            url=r"^{prefix}/delete-bulk{trailing_slash}$",
            mapping={
                "post": "delete_bulk",
            },
            name="{basename}-delete-bulk",
            detail=False,
            initkwargs={"suffix": "Delete List"},
        ),
    )
