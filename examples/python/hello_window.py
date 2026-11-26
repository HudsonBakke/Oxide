from oxide.app import (
    AppConfig,
    Game,
    GameTime,
)
from oxide.content import (
    ContentManager,
    Material,
    Mesh,
)
from oxide.graphics import (
    Camera,
    Color,
    DirectionalLight,
    Graphics,
)
from oxide.math import (
    Angle,
    Transform,
    Vec3,
)


class MyGame(Game):
    def __init__(self) -> None:
        super().__init__()

        self.camera = Camera()
        self.content = ContentManager()

        self.light = DirectionalLight(
            Vec3(1.0, -1.0, -1.0),
            Color.WHITE,
            1.0,
        )

        self.mesh: Mesh | None = None
        self.material: Material | None = None
        self.transform = Transform()

    def initialize(self) -> None:
        print("Python Initialize!")

        self.camera.position = Vec3(
            0.0,
            0.0,
            5.0,
        )

        self.mesh = self.content.load(
            Mesh,
            "assets/textured_material_cube.glb",
        )

        self.material = self.content.load(
            Material,
            "assets/textured_material_cube.glb",
        )

        print(
            "Loaded mesh:",
            self.mesh.vertex_count,
            "vertices,",
            self.mesh.index_count,
            "indices",
        )

    def update(
        self,
        time: GameTime,
    ) -> None:
        self.transform.rotate_yaw_by(Angle.degrees(45.0 * time.delta_seconds))

    def draw(
        self,
        graphics: Graphics,
    ) -> None:
        graphics.bind_camera(
            self.camera,
            graphics.full_rect,
        )

        graphics.add_light(self.light)

        if self.mesh is not None and self.material is not None:
            graphics.draw_mesh(
                self.mesh,
                self.material,
                self.transform,
            )


MyGame().run(
    AppConfig(
        title="Oxide Python Model Test",
        width=1280,
        height=720,
    )
)
