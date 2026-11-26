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

    public Vec2 Position { get; }
    public Vec2 Delta { get; }
    public Vec2 ScrollDelta { get; }

    internal MouseState(NativeMouseState state)
    {
        _current = state.CurrentButtons;
        _previous = state.PreviousButtons;
        Position = state.Position;
        Delta = state.Delta;
        ScrollDelta = state.ScrollDelta;
    }

    public bool IsButtonDown(MouseButtons button) =>
        IsSet((int)button);

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

    public bool IsOtherButtonDown(ushort button) =>
        IsSet(5 + button);

    public bool IsOtherButtonUp(ushort button) =>
        !IsOtherButtonDown(button);

    public bool IsOtherButtonPressed(ushort button) =>
        IsSet(_current, 5 + button)
        && !IsSet(_previous, 5 + button);

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
