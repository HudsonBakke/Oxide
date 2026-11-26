from oxide.app import AppConfig, Game
from oxide.collision import collides
from oxide.content import ContentManager, Material, Mesh
from oxide.graphics import Camera, Color, DirectionalLight, Graphics
from oxide.math import Angle, Transform, Vec3


class ParityGame(Game):
    def __init__(self) -> None:
        super().__init__()

        self.camera = Camera()
        self.content = ContentManager()
        self.light = DirectionalLight(
            Vec3(1.0, -1.0, -1.0),
            Color.WHITE,
            1.0,
        )

        self.parent: Mesh | None = None
        self.child: Mesh | None = None
        self.material: Material | None = None

        self.parent_transform = Transform()
        self.child_transform = Transform()

    def initialize(self) -> None:
        self.camera.position = Vec3(0.0, 0.0, 6.0)

        self.parent = self.content.load(
            Mesh,
            "assets/blender_cube.glb",
        )
        self.child = self.content.load(
            Mesh,
            "assets/blender_cube.glb",
        )
        self.material = self.content.load(
            Material,
            "assets/textured_material_cube.glb",
        )

        self.parent_transform.position = Vec3(
            -1.0,
            0.0,
            0.0,
        )

        self.child_transform = Transform.from_attachment(
            self.parent,
            self.parent_transform,
            Vec3(1.0, 0.0, 0.0),
            self.child,
            Vec3(-1.0, 0.0, 0.0),
            Vec3(1.0, 0.0, 0.0),
            Angle.degrees(45.0),
            Vec3(1.0, 1.0, 1.0),
        )

        print(
            "Attached cubes collide:",
            collides(
                self.parent,
                self.parent_transform,
                self.child,
                self.child_transform,
            ),
        )

    def draw(self, graphics: Graphics) -> None:
        graphics.bind_camera(
            self.camera,
            graphics.full_rect,
        )
        graphics.add_light(self.light)

        if (
            self.parent is not None
            and self.child is not None
            and self.material is not None
        ):
            graphics.draw_mesh(
                self.parent,
                self.material,
                self.parent_transform,
            )
            graphics.draw_mesh(
                self.child,
                self.material,
                self.child_transform,
            )


ParityGame().run(
    AppConfig(
        title="Oxide Python Parity Test",
        width=1280,
        height=720,
    )
)
