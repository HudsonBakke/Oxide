"""Physical keyboard and mouse snapshots with per-frame edge queries."""

from __future__ import annotations

import ctypes
from enum import IntEnum

from . import _native
from .math import Vec2


class Keys(IntEnum):
    """Physical keyboard keys, independent of text input and keyboard layout."""

    #: Physical A key.
    A = 0
    #: Physical B key.
    B = 1
    #: Physical C key.
    C = 2
    #: Physical D key.
    D = 3
    #: Physical E key.
    E = 4
    #: Physical F key.
    F = 5
    #: Physical G key.
    G = 6
    #: Physical H key.
    H = 7
    #: Physical I key.
    I = 8
    #: Physical J key.
    J = 9
    #: Physical K key.
    K = 10
    #: Physical L key.
    L = 11
    #: Physical M key.
    M = 12
    #: Physical N key.
    N = 13
    #: Physical O key.
    O = 14
    #: Physical P key.
    P = 15
    #: Physical Q key.
    Q = 16
    #: Physical R key.
    R = 17
    #: Physical S key.
    S = 18
    #: Physical T key.
    T = 19
    #: Physical U key.
    U = 20
    #: Physical V key.
    V = 21
    #: Physical W key.
    W = 22
    #: Physical X key.
    X = 23
    #: Physical Y key.
    Y = 24
    #: Physical Z key.
    Z = 25
    #: Top-row digit 0 key.
    KEY_0 = 26
    #: Top-row digit 1 key.
    KEY_1 = 27
    #: Top-row digit 2 key.
    KEY_2 = 28
    #: Top-row digit 3 key.
    KEY_3 = 29
    #: Top-row digit 4 key.
    KEY_4 = 30
    #: Top-row digit 5 key.
    KEY_5 = 31
    #: Top-row digit 6 key.
    KEY_6 = 32
    #: Top-row digit 7 key.
    KEY_7 = 33
    #: Top-row digit 8 key.
    KEY_8 = 34
    #: Top-row digit 9 key.
    KEY_9 = 35
    #: Escape key.
    ESCAPE = 36
    #: Space key.
    SPACE = 37
    #: Enter key.
    ENTER = 38
    #: Tab key.
    TAB = 39
    #: Backspace key.
    BACKSPACE = 40
    #: Left shift key.
    LEFT_SHIFT = 41
    #: Right shift key.
    RIGHT_SHIFT = 42
    #: Left control key.
    LEFT_CONTROL = 43
    #: Right control key.
    RIGHT_CONTROL = 44
    #: Left alt key.
    LEFT_ALT = 45
    #: Right alt key.
    RIGHT_ALT = 46
    #: Up key.
    UP = 47
    #: Down key.
    DOWN = 48
    #: Left key.
    LEFT = 49
    #: Right key.
    RIGHT = 50
    #: Insert key.
    INSERT = 51
    #: Delete key.
    DELETE = 52
    #: Home key.
    HOME = 53
    #: End key.
    END = 54
    #: Page up key.
    PAGE_UP = 55
    #: Page down key.
    PAGE_DOWN = 56
    #: F1 key.
    F1 = 57
    #: F2 key.
    F2 = 58
    #: F3 key.
    F3 = 59
    #: F4 key.
    F4 = 60
    #: F5 key.
    F5 = 61
    #: F6 key.
    F6 = 62
    #: F7 key.
    F7 = 63
    #: F8 key.
    F8 = 64
    #: F9 key.
    F9 = 65
    #: F10 key.
    F10 = 66
    #: F11 key.
    F11 = 67
    #: F12 key.
    F12 = 68


class MouseButtons(IntEnum):
    """Named mouse buttons; use the additional-button queries for numeric OS button indices."""

    #: Left mouse button.
    LEFT = 0
    #: Right mouse button.
    RIGHT = 1
    #: Middle mouse button.
    MIDDLE = 2
    #: Back mouse button.
    BACK = 3
    #: Forward mouse button.
    FORWARD = 4


class KeyboardState:
    """Current and previous physical key state for held and per-frame transition queries."""

    def __init__(
        self,
        native: _native.NativeKeyboardState,
    ) -> None:
        """Creates a snapshot from current and previous native key bitsets."""
        self._current = (
            native.current0,
            native.current1,
        )
        self._previous = (
            native.previous0,
            native.previous1,
        )

    def is_key_down(self, key: Keys) -> bool:
        """Whether the physical key is currently held."""
        return self._is_set(
            self._current,
            int(key),
        )

    def is_key_up(self, key: Keys) -> bool:
        """Whether the physical key is currently released."""
        return not self.is_key_down(key)

    def is_key_pressed(self, key: Keys) -> bool:
        """Whether the physical key changed from released to held this frame."""
        index = int(key)
        return self._is_set(self._current, index) and not self._is_set(
            self._previous, index
        )

    def is_key_released(self, key: Keys) -> bool:
        """Whether the physical key changed from held to released this frame."""
        index = int(key)
        return not self._is_set(self._current, index) and self._is_set(
            self._previous, index
        )

    @staticmethod
    def _is_set(
        words: tuple[int, int],
        index: int,
    ) -> bool:
        if not 0 <= index < 128:
            return False

        return (words[index // 64] & (1 << (index % 64))) != 0


class Keyboard:
    """Physical-key snapshots. Focus loss releases held keys through the next active frame.

    Synthetic focus presses and auto-repeat do not create fresh press events.
    """

    @staticmethod
    def get_state() -> KeyboardState:
        """Returns an input snapshot; pressed/released queries compare with the previous frame."""
        native = _native.NativeKeyboardState()

        _native.check(_native.lib.oxide_keyboard_get_state(ctypes.byref(native)))

        return KeyboardState(native)


class MouseState:
    """Mouse snapshot with held buttons, per-frame transitions, position, raw motion, and scrolling.

    Attributes:
        position: Cursor position in physical pixels relative to the window's client-area top-left.
        delta: Raw mouse motion accumulated this frame in device-reported units.
        scroll_delta: Scroll accumulated this frame, in lines or pixels according to the OS event.
    """

    def __init__(
        self,
        native: _native.NativeMouseState,
    ) -> None:
        """Creates a snapshot of native mouse buttons, position, raw motion, and scrolling."""
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
        """Whether the mouse button is currently held."""
        return self._is_set(
            self._current,
            int(button),
        )

    def is_button_up(
        self,
        button: MouseButtons,
    ) -> bool:
        """Whether the mouse button is currently released."""
        return not self.is_button_down(button)

    def is_button_pressed(
        self,
        button: MouseButtons,
    ) -> bool:
        """Whether the mouse button changed from released to held this frame."""
        index = int(button)
        return self._is_set(self._current, index) and not self._is_set(
            self._previous, index
        )

    def is_button_released(
        self,
        button: MouseButtons,
    ) -> bool:
        """Whether the mouse button changed from held to released this frame."""
        index = int(button)
        return not self._is_set(self._current, index) and self._is_set(
            self._previous, index
        )

    def is_other_button_down(
        self,
        button: int,
    ) -> bool:
        """Whether an additional OS mouse button, identified by its numeric index, is held."""
        return self._is_set(
            self._current,
            5 + button,
        )

    def is_other_button_up(
        self,
        button: int,
    ) -> bool:
        """Whether an additional OS mouse button, identified by its numeric index, is released."""
        return not self.is_other_button_down(button)

    def is_other_button_pressed(
        self,
        button: int,
    ) -> bool:
        """Whether an additional OS mouse button became held this frame."""
        index = 5 + button
        return self._is_set(self._current, index) and not self._is_set(
            self._previous, index
        )

    def is_other_button_released(
        self,
        button: int,
    ) -> bool:
        """Whether an additional OS mouse button became released this frame."""
        index = 5 + button
        return not self._is_set(self._current, index) and self._is_set(
            self._previous, index
        )

    @staticmethod
    def _is_set(value: int, index: int) -> bool:
        return 0 <= index < 64 and (value & (1 << index)) != 0


class Mouse:
    """Mouse snapshots. Focus loss releases buttons and clears motion/scroll; unfocused input is ignored."""

    @staticmethod
    def get_state() -> MouseState:
        """Returns an input snapshot; pressed/released queries compare with the previous frame."""
        native = _native.NativeMouseState()

        _native.check(_native.lib.oxide_mouse_get_state(native))

        return MouseState(native)
