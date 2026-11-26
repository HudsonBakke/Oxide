"""Exceptions and native status codes reported by the Oxide bindings."""

from __future__ import annotations

from enum import IntEnum


class OxideStatus(IntEnum):
    """Native operation status; zero indicates success and nonzero values identify failure categories."""

    #: Operation completed successfully.
    OK = 0
    #: An argument or resource handle is invalid.
    INVALID_ARGUMENT = 1
    #: The current state does not permit the operation.
    INVALID_OPERATION = 2
    #: Asset validation, import, or content I/O failed.
    CONTENT_ERROR = 3
    #: Graphics initialization or rendering failed.
    GRAPHICS_ERROR = 4
    #: An unexpected internal failure occurred.
    INTERNAL_ERROR = 5


class OxideError(RuntimeError):
    """Base exception carrying a native status code and diagnostic message.

    Attributes:
        status: Native status code associated with the failure."""

    def __init__(
        self,
        status: OxideStatus,
        message: str,
    ) -> None:
        """Creates an exception with a native status code and diagnostic message."""
        super().__init__(message)
        self.status = status


class InvalidArgumentError(OxideError):
    """Raised when an argument violates a native API requirement."""

    pass


class InvalidOperationError(OxideError):
    """Raised when an operation is invalid for the current engine state."""

    pass


class ContentError(OxideError):
    """Raised when asset validation, importing, or content I/O fails."""

    pass


class GraphicsError(OxideError):
    """Raised when a native graphics operation fails."""

    pass


class InternalError(OxideError):
    """Raised when the native engine reports an unexpected internal failure."""

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
    """Returns the exception class for a native status, falling back to OxideError."""
    return _ERROR_TYPES.get(
        status,
        OxideError,
    )
