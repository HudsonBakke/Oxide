using Oxide.Math;
using Oxide.Native;

namespace Oxide.Input;

/// <summary>Named mouse buttons; use the additional-button queries for numeric OS button indices.</summary>
public enum MouseButtons
{
    /// <summary>Left mouse button.</summary>
    Left = 0,
    /// <summary>Right mouse button.</summary>
    Right = 1,
    /// <summary>Middle mouse button.</summary>
    Middle = 2,
    /// <summary>Back mouse button.</summary>
    Back = 3,
    /// <summary>Forward mouse button.</summary>
    Forward = 4,
}

/// <summary>Access to button, cursor-motion, and scroll snapshots.</summary>
public static class Mouse
{
    /// <summary>Returns an input snapshot; pressed/released queries compare with the previous frame.</summary>
    public static MouseState GetState()
    {
        NativeMethods.Check(NativeMethods.oxide_mouse_get_state(out NativeMouseState state));

        return new MouseState(state);
    }
}

/// <summary>Mouse snapshot with held buttons, per-frame transitions, position, raw motion, and scrolling.</summary>
public readonly struct MouseState
{
    private readonly ulong _current;
    private readonly ulong _previous;
    /// <summary>Cursor position in physical pixels relative to the window's client-area top-left.</summary>
    public Vec2 Position { get; }
    /// <summary>Raw mouse motion accumulated this frame in device-reported units.</summary>
    public Vec2 Delta { get; }
    /// <summary>Scroll accumulated this frame, in lines or pixels according to the OS event.</summary>
    public Vec2 ScrollDelta { get; }

    internal MouseState(NativeMouseState state)
    {
        _current = state.CurrentButtons;
        _previous = state.PreviousButtons;
        Position = state.Position;
        Delta = state.Delta;
        ScrollDelta = state.ScrollDelta;
    }

    /// <summary>Whether the mouse button is currently held.</summary>
    public bool IsButtonDown(MouseButtons button) => IsSet((int)button);
    /// <summary>Whether the mouse button is currently released.</summary>
    public bool IsButtonUp(MouseButtons button) => !IsButtonDown(button);
    /// <summary>Whether the mouse button changed from released to held this frame.</summary>
    public bool IsButtonPressed(MouseButtons button) => IsSet(_current, (int)button) &&
                                                        !IsSet(_previous, (int)button);
    /// <summary>Whether the mouse button changed from held to released this frame.</summary>
    public bool IsButtonReleased(MouseButtons button) => !IsSet(_current, (int)button) &&
                                                         IsSet(_previous, (int)button);
    /// <summary>Whether an additional OS mouse button, identified by its numeric index, is held.</summary>
    public bool IsOtherButtonDown(ushort button) => IsSet(5 + button);
    /// <summary>Whether an additional OS mouse button, identified by its numeric index, is released.</summary>
    public bool IsOtherButtonUp(ushort button) => !IsOtherButtonDown(button);
    /// <summary>Whether an additional OS mouse button became held this frame.</summary>
    public bool IsOtherButtonPressed(ushort button) => IsSet(_current, 5 + button) &&
                                                       !IsSet(_previous, 5 + button);
    /// <summary>Whether an additional OS mouse button became released this frame.</summary>
    public bool IsOtherButtonReleased(ushort button) => !IsSet(_current, 5 + button) &&
                                                        IsSet(_previous, 5 + button);

    private bool IsSet(int index) => IsSet(_current, index);

    private static bool IsSet(ulong value, int index) => (uint)index < 64 &&
                                                         (value & (1UL << index)) != 0;
}
