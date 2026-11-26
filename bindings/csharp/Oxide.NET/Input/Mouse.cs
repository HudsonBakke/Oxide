using Oxide.Math;
using Oxide.Native;

namespace Oxide.Input;

/// <summary>Named mouse buttons recognized by Oxide.</summary>
public enum MouseButtons
{
    Left = 0,
    Right = 1,
    Middle = 2,
    Back = 3,
    Forward = 4,
}

/// <summary>Provides snapshots of mouse input.</summary>
public static class Mouse
{
    /// <summary>Returns an input snapshot for the current frame.</summary>
    public static MouseState GetState()
    {
        NativeMethods.Check(
            NativeMethods.oxide_mouse_get_state(
                out NativeMouseState state
            )
        );

        return new MouseState(state);
    }
}

/// <summary>Mouse snapshot containing button states, position, motion, and scroll deltas.</summary>
public readonly struct MouseState
{
    private readonly ulong _current;
    private readonly ulong _previous;

    /// <summary>Current cursor position in window coordinates.</summary>
    public Vec2 Position { get; }
    /// <summary>Cursor movement accumulated during the current frame.</summary>
    public Vec2 Delta { get; }
    /// <summary>Scroll-wheel movement accumulated during the current frame.</summary>
    public Vec2 ScrollDelta { get; }

    internal MouseState(NativeMouseState state)
    {
        _current = state.CurrentButtons;
        _previous = state.PreviousButtons;
        Position = state.Position;
        Delta = state.Delta;
        ScrollDelta = state.ScrollDelta;
    }

    /// <summary>Returns whether the button is held.</summary>
    public bool IsButtonDown(MouseButtons button) =>
        IsSet((int)button);

    /// <summary>Returns whether the button is not held.</summary>
    public bool IsButtonUp(MouseButtons button) =>
        !IsButtonDown(button);

        /// <summary>Returns true only on the frame the button transitions from up to down.</summary>
public bool IsButtonPressed(MouseButtons button) =>
        IsSet(_current, (int)button)
        && !IsSet(_previous, (int)button);

        /// <summary>Returns true only on the frame the button transitions from down to up.</summary>
public bool IsButtonReleased(MouseButtons button) =>
        !IsSet(_current, (int)button)
        && IsSet(_previous, (int)button);

    /// <summary>Returns whether an additional mouse button is held.</summary>
    public bool IsOtherButtonDown(ushort button) =>
        IsSet(5 + button);

    /// <summary>Returns whether an additional mouse button is not held.</summary>
    public bool IsOtherButtonUp(ushort button) =>
        !IsOtherButtonDown(button);

    /// <summary>Returns true only when an additional button transitions from up to down.</summary>
    public bool IsOtherButtonPressed(ushort button) =>
        IsSet(_current, 5 + button)
        && !IsSet(_previous, 5 + button);

    /// <summary>Returns true only when an additional button transitions from down to up.</summary>
    public bool IsOtherButtonReleased(ushort button) =>
        !IsSet(_current, 5 + button)
        && IsSet(_previous, 5 + button);

    private bool IsSet(int index) =>
        IsSet(_current, index);

    private static bool IsSet(
        ulong value,
        int index
    ) =>
        (uint)index < 64
        && (value & (1UL << index)) != 0;
}
