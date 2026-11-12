using System;

using Oxide.Native;

namespace Oxide;

public sealed class OxideException
    : Exception
{
    internal OxideException(
        OxideStatus status,
        string message
    )
        : base(message)
    {
        Status = status;
    }

    internal OxideStatus Status {
        get;
    }
}