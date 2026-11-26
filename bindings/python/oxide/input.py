from __future__ import annotations

import ctypes
from enum import IntEnum

from . import _native
from .math import Vec2


class Keys(IntEnum):
    A = 0
    B = 1
    C = 2
    D = 3
    E = 4
    F = 5
    G = 6
    H = 7
    I = 8
    J = 9
    K = 10
    L = 11
    M = 12
    N = 13
    O = 14
    P = 15
    Q = 16
    R = 17
    S = 18
    T = 19
    U = 20
    V = 21
    W = 22
    X = 23
    Y = 24
    Z = 25
    KEY_0 = 26
    KEY_1 = 27
    KEY_2 = 28
    KEY_3 = 29
    KEY_4 = 30
    KEY_5 = 31
    KEY_6 = 32
    KEY_7 = 33
    KEY_8 = 34
    KEY_9 = 35
    ESCAPE = 36
    SPACE = 37
    ENTER = 38
    TAB = 39
    BACKSPACE = 40
    LEFT_SHIFT = 41
    RIGHT_SHIFT = 42
    LEFT_CONTROL = 43
    RIGHT_CONTROL = 44
    LEFT_ALT = 45
    RIGHT_ALT = 46
    UP = 47
    DOWN = 48
    LEFT = 49
    RIGHT = 50
    INSERT = 51
    DELETE = 52
    HOME = 53
    END = 54
    PAGE_UP = 55
    PAGE_DOWN = 56
    F1 = 57
    F2 = 58
    F3 = 59
    F4 = 60
    F5 = 61
    F6 = 62
    F7 = 63
    F8 = 64
    F9 = 65
    F10 = 66
    F11 = 67
    F12 = 68


class MouseButtons(IntEnum):
    LEFT = 0
    RIGHT = 1
    MIDDLE = 2
    BACK = 3
    FORWARD = 4


class KeyboardState:
    """Keyboard snapshot supporting held-state and one-frame edge queries."""
    def __init__(
        self,
        native: _native.NativeKeyboardState,
    ) -> None:
        self._current = (
            native.current0,
            native.current1,
        )
        self._previous = (
            native.previous0,
            native.previous1,
        )

    def is_key_down(self, key: Keys) -> bool:
        """Return whether the key is held."""
        return self._is_set(
            self._current,
            int(key),
        )

    def is_key_up(self, key: Keys) -> bool:
        """Return whether the key is not held."""
        return not self.is_key_down(key)

    def is_key_pressed(self, key: Keys) -> bool:
        """Return True only when the key transitions from up to down this frame."""
        index = int(key)
        return (
            self._is_set(self._current, index)
            and not self._is_set(self._previous, index)
        )

    def is_key_released(self, key: Keys) -> bool:
        """Return True only when the key transitions from down to up this frame."""
        index = int(key)
        return (
            not self._is_set(self._current, index)
            and self._is_set(self._previous, index)
        )

    @staticmethod
    def _is_set(
        words: tuple[int, int],
        index: int,
    ) -> bool:
        if not 0 <= index < 128:
            return False

        return (
            words[index // 64]
            & (1 << (index % 64))
        ) != 0


class Keyboard:
    """Entry point for retrieving the current keyboard snapshot."""
    @staticmethod
    def get_state() -> KeyboardState:
        """Return the current input snapshot."""
        native = _native.NativeKeyboardState()

        _native.check(
            _native.lib.oxide_keyboard_get_state(
                ctypes.byref(native)
            )
        )

        return KeyboardState(native)


class MouseState:
    """Mouse snapshot containing button states, position, motion, and scrolling."""
    def __init__(
        self,
        native: _native.NativeMouseState,
    ) -> None:
        self._current = native.current_buttons
        self._previous = native.previous_buttons
        self.position = Vec2(
            native.position.x,
            native.position.y,
        )
        self.delta = Vec2(
            native.delta.x,
            native.delta.y,
        )
        self.scroll_delta = Vec2(
            native.scroll_delta.x,
            native.scroll_delta.y,
        )

    def is_button_down(
        self,
        button: MouseButtons,
    ) -> bool:
        """Return whether the button is held."""
        return self._is_set(
            self._current,
            int(button),
        )

    def is_button_up(
        self,
        button: MouseButtons,
    ) -> bool:
        """Return whether the button is not held."""
        return not self.is_button_down(button)

    def is_button_pressed(
        self,
        button: MouseButtons,
    ) -> bool:
        """Return True only when the button transitions from up to down this frame."""
        index = int(button)
        return (
            self._is_set(self._current, index)
            and not self._is_set(self._previous, index)
        )

    def is_button_released(
        self,
        button: MouseButtons,
    ) -> bool:
        """Return True only when the button transitions from down to up this frame."""
        index = int(button)
        return (
            not self._is_set(self._current, index)
            and self._is_set(self._previous, index)
        )

    def is_other_button_down(
        self,
        button: int,
    ) -> bool:
        """Public Oxide is other button down operation."""
        return self._is_set(
            self._current,
            5 + button,
        )

    def is_other_button_up(
        self,
        button: int,
    ) -> bool:
        """Public Oxide is other button up operation."""
        return not self.is_other_button_down(button)

    def is_other_button_pressed(
        self,
        button: int,
    ) -> bool:
        """Public Oxide is other button pressed operation."""
        index = 5 + button
        return (
            self._is_set(self._current, index)
            and not self._is_set(self._previous, index)
        )

    def is_other_button_released(
        self,
        button: int,
    ) -> bool:
        """Public Oxide is other button released operation."""
        index = 5 + button
        return (
            not self._is_set(self._current, index)
            and self._is_set(self._previous, index)
        )

    @staticmethod
    def _is_set(value: int, index: int) -> bool:
        return (
            0 <= index < 64
            and (value & (1 << index)) != 0
        )


class Mouse:
    """Entry point for retrieving the current mouse snapshot."""
    @staticmethod
    def get_state() -> MouseState:
        """Return the current input snapshot."""
        native = _native.NativeMouseState()

        _native.check(
            _native.lib.oxide_mouse_get_state(
                native
            )
        )

        return MouseState(native)
