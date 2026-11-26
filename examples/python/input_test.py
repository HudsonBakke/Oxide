from oxide.app import AppConfig, Game, GameTime
from oxide.input import (
    Keyboard,
    Keys,
    Mouse,
    MouseButtons,
)


class InputGame(Game):
    def update(self, time: GameTime) -> None:
        keyboard = Keyboard.get_state()
        mouse = Mouse.get_state()

        if keyboard.is_key_pressed(Keys.SPACE):
            print("Space pressed")

        if keyboard.is_key_released(Keys.SPACE):
            print("Space released")

        if keyboard.is_key_down(Keys.W):
            print("W is down")

        if mouse.is_button_pressed(MouseButtons.LEFT):
            print("Left mouse pressed at", mouse.position)

        if mouse.delta.x != 0.0 or mouse.delta.y != 0.0:
            print("Mouse delta:", mouse.delta)

        if (
            mouse.scroll_delta.x != 0.0
            or mouse.scroll_delta.y != 0.0
        ):
            print("Scroll:", mouse.scroll_delta)


InputGame().run(
    AppConfig(
        title="Oxide Python Input Test",
        width=800,
        height=600,
    )
)
