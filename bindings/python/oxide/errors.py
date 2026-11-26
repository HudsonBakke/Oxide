from __future__ import annotations

from enum import IntEnum


class OxideStatus(IntEnum):
    OK = 0
    INVALID_ARGUMENT = 1
    INVALID_OPERATION = 2
    CONTENT_ERROR = 3
    GRAPHICS_ERROR = 4
    INTERNAL_ERROR = 5


class OxideError(RuntimeError):
    def __init__(
        self,
        status: OxideStatus,
        message: str,
    ) -> None:
        super().__init__(message)
        self.status = status


class InvalidArgumentError(OxideError):
    pass


class InvalidOperationError(OxideError):
    pass


class ContentError(OxideError):
    pass


class GraphicsError(OxideError):
    pass


class InternalError(OxideError):
    pass


_ERROR_TYPES = {
    OxideStatus.INVALID_ARGUMENT: InvalidArgumentError,
    OxideStatus.INVALID_OPERATION: InvalidOperationError,
    OxideStatus.CONTENT_ERROR: ContentError,
    OxideStatus.GRAPHICS_ERROR: GraphicsError,
    OxideStatus.INTERNAL_ERROR: InternalError,
}


def error_type_for(
    status: OxideStatus,
) -> type[OxideError]:
    return _ERROR_TYPES.get(
        status,
        OxideError,
    )
