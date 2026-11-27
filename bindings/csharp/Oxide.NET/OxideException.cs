using System;

using Oxide.Native;

namespace Oxide;

/// <summary>Category of a failure returned by the native engine.</summary>
public enum OxideErrorKind
{
    /// <summary>An argument or resource handle is invalid.</summary>
    InvalidArgument = 1,
    /// <summary>The current state does not permit the operation.</summary>
    InvalidOperation = 2,
    /// <summary>Asset validation, import, or content I/O failed.</summary>
    Content = 3,
    /// <summary>Graphics initialization or rendering failed.</summary>
    Graphics = 4,
    /// <summary>An unexpected internal failure occurred.</summary>
    Internal = 5,
    /// <summary>Audio output initialization or playback failed.</summary>
    Audio = 6,
}

/// <summary>Exception raised when the native engine returns a non-success status.</summary>
public sealed class OxideException : Exception
{
    internal OxideException(OxideStatus status, string message) : base(message)
    {
        Kind = (OxideErrorKind)(int)status;
    }

    /// <summary>Category of the native failure that produced this exception.</summary>
    public OxideErrorKind Kind { get; }
}
