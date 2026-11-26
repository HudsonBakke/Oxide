from __future__ import annotations

from dataclasses import dataclass

from . import _native
from .graphics import Graphics


@dataclass
class AppConfig:
    """Window configuration used when starting an Oxide game."""
    title: str = "Oxide"
    width: int = 1280
    height: int = 720


@dataclass(frozen=True)
class GameTime:
    """Elapsed and total timing information for an update callback."""
    delta_seconds: float
    total_seconds: float


class Game:
    """Base class for an Oxide application; override lifecycle methods to implement a game."""
    def __init__(self) -> None:
        self._has_run = False

    def initialize(self) -> None:
        """Called once after Oxide initializes the application."""
        pass

    def update(
        self,
        time: GameTime,
    ) -> None:
        """Called once per update with timing information."""
        pass

    def draw(
        self,
        graphics: Graphics,
    ) -> None:
        """Called each frame to submit rendering commands."""
        pass

    @property
    def window_width(self) -> int:
        """Public Oxide window width operation."""
        return _native.lib.oxide_app_window_width()

    @property
    def window_height(self) -> int:
        """Public Oxide window height operation."""
        return _native.lib.oxide_app_window_height()

    @property
    def is_window_focused(self) -> bool:
        """Public Oxide is window focused operation."""
        return _native.lib.oxide_app_is_window_focused() != 0

    @property
    def is_fullscreen(self) -> bool:
        """Public Oxide is fullscreen operation."""
        return _native.lib.oxide_app_is_fullscreen() != 0

    @is_fullscreen.setter
    def is_fullscreen(self, value: bool) -> None:
        """Public Oxide is fullscreen operation."""
        _native.check(
            _native.lib.oxide_app_set_fullscreen(
                1 if value else 0
            )
        )

    def exit(self) -> None:
        """Request that the running application exit."""
        _native.check(
            _native.lib.oxide_app_exit()
        )

    def set_window_name(
        self,
        name: str,
    ) -> None:
        """Change the native window title."""
        if not isinstance(name, str):
            raise TypeError("name must be a string")

        _native.check(
            _native.lib.oxide_app_set_window_name(
                name.encode("utf-8")
            )
        )

    def set_window_logo(
        self,
        texture,
    ) -> None:
        """Change the native window icon using a texture."""
        from .content import Texture

        if not isinstance(texture, Texture):
            raise TypeError("texture must be a Texture")

        _native.check(
            _native.lib.oxide_app_set_window_logo(
                texture._handle
            )
        )

    def set_preferred_backbuffer_size(
        self,
        width: int,
        height: int,
    ) -> None:
        """Request a new backbuffer size."""
        if width <= 0 or height <= 0:
            raise ValueError(
                "backbuffer dimensions must be greater than zero"
            )

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
        """Public Oxide set preferred backbuffer width operation."""
        self.set_preferred_backbuffer_size(
            width,
            self.window_height,
        )

    def set_preferred_backbuffer_height(
        self,
        height: int,
    ) -> None:
        """Public Oxide set preferred backbuffer height operation."""
        self.set_preferred_backbuffer_size(
            self.window_width,
            height,
        )

    def set_cursor_visible(
        self,
        visible: bool,
    ) -> None:
        """Show or hide the system cursor."""
        _native.check(
            _native.lib.oxide_app_set_cursor_visible(
                1 if visible else 0
            )
        )

    def set_cursor_grabbed(
        self,
        grabbed: bool,
    ) -> None:
        """Confine or release the cursor from the window."""
        _native.check(
            _native.lib.oxide_app_set_cursor_grabbed(
                1 if grabbed else 0
            )
        )

    def run(
        self,
        config: AppConfig | None = None,
    ) -> None:
        """Start the Oxide application loop; a Game instance can run only once."""
        if self._has_run:
            raise RuntimeError(
                "An Oxide Game instance can only be run once."
            )

        self._has_run = True

        if config is None:
            config = AppConfig()

        if not isinstance(config.title, str):
            raise TypeError(
                "title must be a string"
            )

        if not 1 <= config.width <= 0xFFFFFFFF:
            raise ValueError(
                "width must be between 1 and 4294967295"
            )

        if not 1 <= config.height <= 0xFFFFFFFF:
            raise ValueError(
                "height must be between 1 and 4294967295"
            )

        title = config.title.encode(
            "utf-8"
        )

        callback_exception: BaseException | None = (
            None
        )

        def capture_exception(
            exception: BaseException,
        ) -> None:
            """Public Oxide capture exception operation."""
            nonlocal callback_exception

            if callback_exception is None:
                callback_exception = exception

        @_native.InitializeCallback
        def on_initialize(
            user_data,
        ) -> None:
            """Public Oxide on initialize operation."""
            if callback_exception is not None:
                return

            try:
                self.initialize()
            except BaseException as exception:
                capture_exception(
                    exception
                )

        @_native.UpdateCallback
        def on_update(
            user_data,
            native_time,
        ) -> None:
            """Public Oxide on update operation."""
            if callback_exception is not None:
                return

            try:
                self.update(
                    GameTime(
                        delta_seconds=(
                            native_time
                            .delta_seconds
                        ),
                        total_seconds=(
                            native_time
                            .total_seconds
                        ),
                    )
                )
            except BaseException as exception:
                capture_exception(
                    exception
                )

        @_native.DrawCallback
        def on_draw(
            user_data,
            graphics_handle,
        ) -> None:
            """Public Oxide on draw operation."""
            if callback_exception is not None:
                return

            graphics = Graphics(
                graphics_handle
            )

            try:
                self.draw(
                    graphics
                )
            except BaseException as exception:
                capture_exception(
                    exception
                )
            finally:
                graphics._invalidate()

        native_config = (
            _native.NativeAppConfig(
                title=title,
                width=config.width,
                height=config.height,
            )
        )

        callbacks = (
            _native.NativeGameCallbacks(
                user_data=None,
                initialize=on_initialize,
                update=on_update,
                draw=on_draw,
            )
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

        _native.check(
            status
        )