using System;

using Oxide.Native;

namespace Oxide;

/// <summary>Base exception reported by the Oxide native runtime.</summary>
public enum OxideErrorKind
{
    InvalidArgument = 1,
    InvalidOperation = 2,
    Content = 3,
    Graphics = 4,
    Internal = 5,
}

public sealed class OxideException
    : Exception
{
    internal OxideException(
        OxideStatus status,
        string message
    )
        : base(message)
    {
        Kind =
            (OxideErrorKind)(int)status;
    }

    public OxideErrorKind Kind {
        get;
    }
}