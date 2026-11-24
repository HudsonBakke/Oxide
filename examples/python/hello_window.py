from oxide.app import (
    AppConfig,
    Game,
    GameTime,
)

from oxide.graphics import (
    Camera,
    Color,
    DirectionalLight,
    Graphics,
)

from oxide.math import Vec3


class MyGame(Game):
    def __init__(self) -> None:
        super().__init__()

        self.camera = Camera()

        self.light = (
            DirectionalLight(
                Vec3(
                    1.0,
                    -1.0,
                    -1.0,
                ),
                Color.WHITE,
                1.0,
            )
        )

    def initialize(self) -> None:
        print(
            "Python Initialize!"
        )

        self.camera.position = Vec3(
            0.0,
            0.0,
            5.0,
        )

        print(
            "Camera position:",
            self.camera.position,
        )

        print(
            "Camera clips:",
            self.camera.near_plane,
            "-",
            self.camera.far_plane,
        )

    def update(
        self,
        time: GameTime,
    ) -> None:
        pass

    def draw(
        self,
        graphics: Graphics,
    ) -> None:
        graphics.bind_camera(
            self.camera,
            graphics.full_rect,
        )

        graphics.add_light(
            self.light
        )


game = MyGame()

game.run(
    AppConfig(
        title="Oxide Python Graphics Test",
        width=1280,
        height=720,
    )
)