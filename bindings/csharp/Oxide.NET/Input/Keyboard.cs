using Oxide.Native;

namespace Oxide.Input;

public static class Keyboard
{
    public static KeyboardState GetState()
    {
        NativeMethods.Check(
            NativeMethods.oxide_keyboard_get_state(
                out NativeKeyboardState state
            )
        );

        return new KeyboardState(state);
    }
}

public readonly struct KeyboardState
{
    private readonly ulong _current0;
    private readonly ulong _current1;
    private readonly ulong _previous0;
    private readonly ulong _previous1;

    internal KeyboardState(
        NativeKeyboardState state
    )
    {
        _current0 = state.Current0;
        _current1 = state.Current1;
        _previous0 = state.Previous0;
        _previous1 = state.Previous1;
    }

    public bool IsKeyDown(Keys key) =>
        IsSet(_current0, _current1, (int)key);

    public bool IsKeyUp(Keys key) =>
        !IsKeyDown(key);

    public bool IsKeyPressed(Keys key) =>
        IsSet(_current0, _current1, (int)key)
        && !IsSet(_previous0, _previous1, (int)key);

    public bool IsKeyReleased(Keys key) =>
        !IsSet(_current0, _current1, (int)key)
        && IsSet(_previous0, _previous1, (int)key);

    private static bool IsSet(
        ulong low,
        ulong high,
        int index
    )
    {
        if ((uint)index >= 128)
        {
            return false;
        }

        return index < 64
            ? (low & (1UL << index)) != 0
            : (high & (1UL << (index - 64))) != 0;
    }
}
