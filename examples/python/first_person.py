import math

from oxide.app import AppConfig, Game, GameTime
from oxide.content import Material, Mesh, Vertex
from oxide.graphics import (
    Camera,
    Color,
    DirectionalLight,
    Graphics,
    PointLight,
    SpotLight,
)
from oxide.input import Keyboard, Keys, Mouse, MouseButtons
from oxide.math import Angle, Quaternion, Transform, Vec2, Vec3


def make_quad() -> Mesh:
    return Mesh(
        [
            Vertex(Vec3(-0.5, 0, -0.5), Vec3(0, 1, 0), Vec2(0, 0)),
            Vertex(Vec3(0.5, 0, -0.5), Vec3(0, 1, 0), Vec2(1, 0)),
            Vertex(Vec3(0.5, 0, 0.5), Vec3(0, 1, 0), Vec2(1, 1)),
            Vertex(Vec3(-0.5, 0, 0.5), Vec3(0, 1, 0), Vec2(0, 1)),
        ],
        [0, 1, 2, 0, 2, 3],
    )


def make_cube() -> Mesh:
    vertices: list[Vertex] = []
    indices: list[int] = []

    faces = [
        (Vec3(0, 0, 1), [(-1, -1, 1), (1, -1, 1), (1, 1, 1), (-1, 1, 1)]),
        (Vec3(0, 0, -1), [(1, -1, -1), (-1, -1, -1), (-1, 1, -1), (1, 1, -1)]),
        (Vec3(1, 0, 0), [(1, -1, 1), (1, -1, -1), (1, 1, -1), (1, 1, 1)]),
        (Vec3(-1, 0, 0), [(-1, -1, -1), (-1, -1, 1), (-1, 1, 1), (-1, 1, -1)]),
        (Vec3(0, 1, 0), [(-1, 1, 1), (1, 1, 1), (1, 1, -1), (-1, 1, -1)]),
        (Vec3(0, -1, 0), [(-1, -1, -1), (1, -1, -1), (1, -1, 1), (-1, -1, 1)]),
    ]

    for normal, points in faces:
        base = len(vertices)
        for x, y, z in points:
            vertices.append(
                Vertex(
                    Vec3(x * 0.5, y * 0.5, z * 0.5),
                    normal,
                    Vec2(),
                )
            )
        indices.extend([base, base + 2, base + 1, base, base + 3, base + 2])

    return Mesh(vertices, indices)


def make_sphere(segments: int, rings: int) -> Mesh:
    vertices: list[Vertex] = []
    indices: list[int] = []

    for ring in range(rings + 1):
        v = ring / rings
        phi = v * math.pi
        y = math.cos(phi)
        radius = math.sin(phi)

        for segment in range(segments + 1):
            u = segment / segments
            theta = u * math.tau
            normal = Vec3(
                radius * math.cos(theta),
                y,
                radius * math.sin(theta),
            )
            vertices.append(Vertex(normal * 0.5, normal, Vec2(u, v)))

    stride = segments + 1
    for ring in range(rings):
        for segment in range(segments):
            a = ring * stride + segment
            b = a + stride
            indices.extend([a, a + 1, b + 1, a, b + 1, b])

    return Mesh(vertices, indices)


class FirstPersonGame(Game):
    def __init__(self) -> None:
        super().__init__()
        self.camera = Camera()
        self.floor = make_quad()
        self.cube = make_cube()
        self.sphere = make_sphere(16, 10)
        self.green = Material.solid(Color.rgb(0.25, 0.55, 0.22))
        self.dark = Material.solid(Color.rgb(0.12, 0.18, 0.12))
        self.brown = Material.solid(Color.rgb(0.45, 0.25, 0.10))
        self.red = Material.solid(Color.rgb(0.65, 0.16, 0.10))
        self.directional_lights = [
            DirectionalLight(Vec3(0, -1, 0), Color.rgb(1.0, 0.75, 0.65), 0.30),
            DirectionalLight(Vec3(0, 1, 0), Color.rgb(0.45, 0.55, 1.0), 0.18),
            DirectionalLight(Vec3(-1, 0, 0), Color.rgb(1.0, 0.35, 0.30), 0.22),
            DirectionalLight(Vec3(1, 0, 0), Color.rgb(0.30, 1.0, 0.45), 0.22),
        ]
        self.sun = PointLight(Vec3(4, 4, -5), Color.rgb(1.0, 0.72, 0.15), 2.5)
        self.flashlight = SpotLight(
            Vec3(-4, 1.2, -3),
            Vec3(0.8, -0.15, -1),
            Angle.degrees(32),
            Color.WHITE,
            5.0,
        )
        self.yellow = Material.solid(Color.rgb(1.0, 0.75, 0.10))
        self.gray = Material.solid(Color.rgb(0.22, 0.24, 0.28))
        self.yaw = 0.0
        self.pitch = 0.0
        self.captured = False

    def initialize(self) -> None:
        self.yellow.emissive_color = Color.rgb(1.0, 0.65, 0.05)
        self.yellow.emissive_strength = 3.0
        self.gray.metallic = 0.75
        self.gray.roughness = 0.28

        self.camera.position = Vec3(0.0, 1.7, 7.0)
        self.set_window_name("Oxide First Person Camera")

    def update(self, time: GameTime) -> None:
        keyboard = Keyboard.get_state()
        mouse = Mouse.get_state()

        if mouse.is_button_pressed(MouseButtons.LEFT) and not self.captured:
            self.captured = True
            self.set_cursor_visible(False)
            self.set_cursor_grabbed(True)

        if keyboard.is_key_pressed(Keys.ESCAPE):
            if self.captured:
                self.captured = False
                self.set_cursor_grabbed(False)
                self.set_cursor_visible(True)
            else:
                self.exit()
            return

        if keyboard.is_key_pressed(Keys.F11):
            self.is_fullscreen = not self.is_fullscreen

        if keyboard.is_key_pressed(Keys.F2):
            self.set_preferred_backbuffer_size(1920, 1080)

        if self.captured:
            self.yaw -= mouse.delta.x * 0.0025
            self.pitch = max(-1.5, min(1.5, self.pitch - mouse.delta.y * 0.0025))

        self.camera.set_euler(
            Angle.from_radians(self.pitch),
            Angle.from_radians(self.yaw),
            Angle.zero(),
        )

        rotation = Quaternion.from_euler(
            Angle.zero(),
            Angle.from_radians(self.yaw),
            Angle.zero(),
        )

        forward = rotation.rotate_vector(Vec3(0, 0, -1))
        right = rotation.rotate_vector(Vec3(1, 0, 0))

        movement = Vec3()
        if keyboard.is_key_down(Keys.W):
            movement = movement + forward
        if keyboard.is_key_down(Keys.S):
            movement = movement - forward
        if keyboard.is_key_down(Keys.D):
            movement = movement + right
        if keyboard.is_key_down(Keys.A):
            movement = movement - right
        if keyboard.is_key_down(Keys.SPACE):
            movement = movement + Vec3(0, 1, 0)
        if keyboard.is_key_down(Keys.LEFT_SHIFT) or keyboard.is_key_down(
            Keys.RIGHT_SHIFT
        ):
            movement = movement - Vec3(0, 1, 0)

        if movement.length > 0.0:
            self.camera.translate_by(movement.normalized() * (5.0 * time.delta_seconds))

    @staticmethod
    def draw_oriented_box(
        graphics: Graphics,
        mesh: Mesh,
        material: Material,
        position: Vec3,
        scale: Vec3,
        rotation: Quaternion,
    ) -> None:
        graphics.draw_mesh(
            mesh,
            material,
            Transform(position=position, scale=scale, rotation=rotation),
        )

    @staticmethod
    def draw_box(
        graphics: Graphics,
        mesh: Mesh,
        material: Material,
        position: Vec3,
        scale: Vec3,
    ) -> None:
        graphics.draw_mesh(
            mesh,
            material,
            Transform(position=position, scale=scale),
        )

    def draw(self, graphics: Graphics) -> None:
        graphics.bind_camera(self.camera, graphics.full_rect)
        for light in self.directional_lights:
            graphics.add_light(light)
        graphics.add_light(self.sun)
        camera_rotation = self.camera.rotation
        camera_position = self.camera.position
        forward3d = camera_rotation.rotate_vector(Vec3(0, 0, -1))
        right3d = camera_rotation.rotate_vector(Vec3(1, 0, 0))
        down3d = camera_rotation.rotate_vector(Vec3(0, -1, 0))
        self.flashlight.position = (
            camera_position + forward3d * 0.7 + right3d * 0.35 + down3d * 0.25
        )
        self.flashlight.direction = forward3d
        graphics.add_light(self.flashlight)

        for x in range(-5, 6):
            for z in range(-5, 6):
                self.draw_box(
                    graphics,
                    self.floor,
                    self.green if (x + z) % 2 == 0 else self.dark,
                    Vec3(x * 2.0, 0.0, z * 2.0),
                    Vec3(2.0, 1.0, 2.0),
                )

        self.draw_box(
            graphics, self.sphere, self.yellow, Vec3(4, 4, -5), Vec3(1.1, 1.1, 1.1)
        )
        self.draw_oriented_box(
            graphics,
            self.cube,
            self.gray,
            camera_position + forward3d * 0.9 + right3d * 0.35 + down3d * 0.25,
            Vec3(0.22, 0.22, 0.8),
            camera_rotation,
        )
        self.draw_oriented_box(
            graphics,
            self.cube,
            self.yellow,
            camera_position + forward3d * 1.3 + right3d * 0.35 + down3d * 0.25,
            Vec3(0.28, 0.28, 0.12),
            camera_rotation,
        )

        self.draw_box(
            graphics, self.cube, self.brown, Vec3(0, 1.5, -6), Vec3(0.22, 3, 0.22)
        )
        self.draw_box(
            graphics, self.cube, self.red, Vec3(0, 2.8, -6), Vec3(1.4, 1.3, 0.55)
        )
        self.draw_box(
            graphics, self.cube, self.brown, Vec3(0, 3.9, -6), Vec3(0.8, 0.8, 0.8)
        )
        self.draw_box(
            graphics, self.cube, self.brown, Vec3(0, 3.0, -6), Vec3(4.0, 0.18, 0.18)
        )
        self.draw_box(
            graphics, self.cube, self.dark, Vec3(0, 4.45, -6), Vec3(1.7, 0.15, 1.2)
        )
        self.draw_box(
            graphics, self.cube, self.dark, Vec3(0, 4.75, -6), Vec3(0.9, 0.55, 0.8)
        )


FirstPersonGame().run(
    AppConfig(
        title="Oxide First Person Camera",
        width=1280,
        height=720,
    )
)
