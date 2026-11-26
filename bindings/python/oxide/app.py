"""Application configuration, frame timing, and overridable lifecycle callbacks."""

from __future__ import annotations

from dataclasses import dataclass

from . import _native
from .graphics import Graphics


@dataclass
class AppConfig:
    """Initial window title and logical-pixel client size; defaults to Oxide at 1280 by 720.

    Attributes:
        title: Initial window title; defaults to Oxide.
        width: Initial client width in logical pixels; must be positive.
        height: Initial client height in logical pixels; must be positive."""

    title: str = "Oxide"
    width: int = 1280
    height: int = 720


@dataclass(frozen=True)
class GameTime:
    """Frame timing in seconds; delta time varies with actual frame duration.

    Attributes:
        delta_seconds: Seconds elapsed since the previous frame's update.
        total_seconds: Seconds elapsed since the application clock started."""

    delta_seconds: float
    total_seconds: float


class Game:
    """Application lifecycle implemented by initialize, update, and draw callbacks."""

    def __init__(self) -> None:
        """Creates an application instance whose lifecycle callbacks may be overridden."""
        self._has_run = False

    def initialize(self) -> None:
        """Called once after the window and graphics are ready; load persistent resources here."""
        pass

    def update(
        self,
        time: GameTime,
    ) -> None:
        """Called once before drawing each frame with elapsed time in seconds; update game logic and read input here."""
        pass

    def draw(
        self,
        graphics: Graphics,
    ) -> None:
        """Called each frame to bind cameras, set lighting, and submit draw calls."""
        pass

    @property
    def window_width(self) -> int:
        """Current client-area width in physical pixels."""
        return _native.lib.oxide_app_window_width()

    @property
    def window_height(self) -> int:
        """Current client-area height in physical pixels."""
        return _native.lib.oxide_app_window_height()

    @property
    def is_window_focused(self) -> bool:
        """Whether the application window currently has keyboard focus."""
        return _native.lib.oxide_app_is_window_focused() != 0

    @property
    def is_fullscreen(self) -> bool:
        """Whether borderless fullscreen is enabled or has been requested."""
        return _native.lib.oxide_app_is_fullscreen() != 0

    @is_fullscreen.setter
    def is_fullscreen(self, value: bool) -> None:
        """Whether borderless fullscreen is enabled or has been requested."""
        _native.check(_native.lib.oxide_app_set_fullscreen(1 if value else 0))

    def exit(self) -> None:
        """Queues a request to leave the application event loop."""
        _native.check(_native.lib.oxide_app_exit())

    def set_window_name(
        self,
        name: str,
    ) -> None:
        """Queues a change to the native window title."""
        if not isinstance(name, str):
            raise TypeError("name must be a string")

        _native.check(_native.lib.oxide_app_set_window_name(name.encode("utf-8")))

    def set_window_logo(
        self,
        texture,
    ) -> None:
        """Queues an icon change using an RGBA8 texture."""
        from .content import Texture

        if not isinstance(texture, Texture):
            raise TypeError("texture must be a Texture")

        _native.check(_native.lib.oxide_app_set_window_logo(texture._handle))

    def set_preferred_backbuffer_size(
        self,
        width: int,
        height: int,
    ) -> None:
        """Requests a client/backbuffer size in physical pixels; both dimensions must be positive."""
        if width <= 0 or height <= 0:
            raise ValueError("backbuffer dimensions must be greater than zero")

        _native.check(
            _native.lib.oxide_app_set_preferred_backbuffer_size(
                width,
                height,
            )
        )

    def set_preferred_backbuffer_width(
        self,
        width: int,
    ) -> None:
        """Requests a physical-pixel width while retaining the current window height."""
        self.set_preferred_backbuffer_size(
            width,
            self.window_height,
        )

    def set_preferred_backbuffer_height(
        self,
        height: int,
    ) -> None:
        """Requests a physical-pixel height while retaining the current window width."""
        self.set_preferred_backbuffer_size(
            self.window_width,
            height,
        )

    def set_cursor_visible(
        self,
        visible: bool,
    ) -> None:
        """Queues a change to the visibility of the OS cursor over the game window."""
        _native.check(_native.lib.oxide_app_set_cursor_visible(1 if visible else 0))

    def set_cursor_grabbed(
        self,
        grabbed: bool,
    ) -> None:
        """Requests cursor locking, falling back to confinement when locking is unavailable."""
        _native.check(_native.lib.oxide_app_set_cursor_grabbed(1 if grabbed else 0))

    def run(
        self,
        config: AppConfig | None = None,
    ) -> None:
        """Runs the blocking application event loop on the calling thread until the window closes or exit is requested."""
        if self._has_run:
            raise RuntimeError("An Oxide Game instance can only be run once.")

        self._has_run = True

        if config is None:
            config = AppConfig()

        if not isinstance(config.title, str):
            raise TypeError("title must be a string")

        if not 1 <= config.width <= 0xFFFFFFFF:
            raise ValueError("width must be between 1 and 4294967295")

        if not 1 <= config.height <= 0xFFFFFFFF:
            raise ValueError("height must be between 1 and 4294967295")

        title = config.title.encode("utf-8")

        callback_exception: BaseException | None = None

        def capture_exception(
            exception: BaseException,
        ) -> None:
            """Retains the first callback exception for propagation after the event loop returns."""
            nonlocal callback_exception

            if callback_exception is None:
                callback_exception = exception

        @_native.InitializeCallback
        def on_initialize(
            user_data,
        ) -> None:
            """Invokes the Python initialization callback behind the native ABI boundary."""
            if callback_exception is not None:
                return

            try:
                self.initialize()
            except BaseException as exception:
                capture_exception(exception)

        @_native.UpdateCallback
        def on_update(
            user_data,
            native_time,
        ) -> None:
            """Converts native timing and invokes the Python update callback."""
            if callback_exception is not None:
                return

            try:
                self.update(
                    GameTime(
                        delta_seconds=(native_time.delta_seconds),
                        total_seconds=(native_time.total_seconds),
                    )
                )
            except BaseException as exception:
                capture_exception(exception)

        @_native.DrawCallback
        def on_draw(
            user_data,
            graphics_handle,
        ) -> None:
            """Invokes drawing with a rendering handle valid only for this callback."""
            if callback_exception is not None:
                return

            graphics = Graphics(graphics_handle)

            try:
                self.draw(graphics)
            except BaseException as exception:
                capture_exception(exception)
            finally:
                graphics._invalidate()

        native_config = _native.NativeAppConfig(
            title=title,
            width=config.width,
            height=config.height,
        )

        callbacks = _native.NativeGameCallbacks(
            user_data=None,
            initialize=on_initialize,
            update=on_update,
            draw=on_draw,
        )

        status = _native.lib.oxide_run(
            native_config,
            callbacks,
        )

        # Keep all CFUNCTYPE objects alive until Rust's event
        # loop has completely returned.
        _ = (
            on_initialize,
            on_update,
            on_draw,
            title,
        )

        if callback_exception is not None:
            raise callback_exception

        _native.check(status)
