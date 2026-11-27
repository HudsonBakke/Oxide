"""Automatic shadows. Space toggles shadows; T cycles box transparency."""
import math

from oxide.app import AppConfig, Game, GameTime
from oxide.content import Material, Mesh, Vertex
from oxide.graphics import (
    Camera, Color, DirectionalLight, DirectionalShadowSettings, Environment, Graphics,
)
from oxide.input import Keyboard, Keys
from oxide.math import Angle, Quaternion, Transform, Vec2, Vec3


def cube() -> Mesh:
    vertices, indices = [], []
    for n, r, u in [
        ((0, 0, 1), (1, 0, 0), (0, 1, 0)),
        ((0, 0, -1), (-1, 0, 0), (0, 1, 0)),
        ((1, 0, 0), (0, 0, -1), (0, 1, 0)),
        ((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
        ((0, 1, 0), (1, 0, 0), (0, 0, -1)),
        ((0, -1, 0), (1, 0, 0), (0, 0, 1)),
    ]:
        normal, right, up = Vec3(*n), Vec3(*r), Vec3(*u)
        base = len(vertices)
        for x, y in [(-1, -1), (1, -1), (1, 1), (-1, 1)]:
            vertices.append(Vertex((normal + right * x + up * y) * 0.5,
                                   normal, Vec2((x + 1) * 0.5, (y + 1) * 0.5)))
        indices.extend([base, base + 2, base + 1, base, base + 3, base + 2])
    return Mesh(vertices, indices)


class Shadows(Game):
    def __init__(self) -> None:
        super().__init__()
        self.camera = Camera()
        position = Vec3(9, 8, 13)
        self.camera.position = position
        self.camera.rotation = Quaternion.from_to(Vec3(0, 0, -1), Vec3(0, 0.5, 0) - position)
        self.mesh = cube()
        self.materials = [Material.solid(Color.rgb(*rgb)) for rgb in
                          [(0.55, 0.58, 0.62), (0.8, 0.18, 0.06), (0.08, 0.35, 0.8),
                           (0.2, 0.8, 0.2), (0.15, 0.7, 0.8), (0.15, 0.7, 0.8), (0.95, 0.75, 0.05)]]
        self.transforms = [
            Transform(position=Vec3(0, -0.1, 0), scale=Vec3(16, 0.2, 16)),
            Transform(position=Vec3(0, 1.5, 0), scale=Vec3(2, 2, 2)),
            Transform(position=Vec3(3, 1.5, -2.5), scale=Vec3(1.5, 3, 1.5)),
            Transform(position=Vec3(-3.5, 1, -3), scale=Vec3(1.5, 2, 1.5)),
            Transform(position=Vec3(-3, 3, 2), scale=Vec3(2, 0.1, 2)),
            Transform(position=Vec3(-2.5, 4, 2.35), scale=Vec3(2, 0.1, 2)),
            Transform(position=Vec3(-1.2, 1, -1.8), scale=Vec3(0.8, 0.8, 0.8)),
        ]
        self.light = DirectionalLight(Vec3(-0.5, -1, -0.35), Color.WHITE, 3)
        self.settings = DirectionalShadowSettings()
        self.environment = Environment(Color.WHITE, 0.12, None, 0, None, Color.rgb(0.04, 0.06, 0.09))
        self.enabled = True
        self.transparency = 0.5

    def update(self, time: GameTime) -> None:
        t = time.total_seconds
        self.transforms[1].position = Vec3(math.sin(t) * 2.5, 1.5 + math.cos(t) * 0.4, 0)
        self.transforms[1].rotation = Quaternion.from_axis_angle(Vec3(0, 1, 0), Angle.from_radians(t * 0.5))
        if Keyboard.get_state().is_key_pressed(Keys.SPACE):
            self.enabled = not self.enabled
        if Keyboard.get_state().is_key_pressed(Keys.T):
            self.transparency = self.transparency + 0.5 if self.transparency < 1 else 0.0
        self.set_window_name(
            f"Oxide Shadows | {'on' if self.enabled else 'off'} | box transparency {self.transparency} | Space toggles, T cycles"
        )

    def draw(self, graphics: Graphics) -> None:
        if self.enabled:
            graphics.set_directional_shadows(self.light, self.settings)
        else:
            graphics.clear_directional_shadows()
        graphics.bind_camera(self.camera, graphics.full_rect)
        graphics.set_environment(self.environment)
        graphics.add_light(self.light)
        transmission = [0, self.transparency, 0, 0.75, 0.5, 0.5, 0]
        for material, transform, transparency in zip(self.materials, self.transforms, transmission):
            if transparency == 0:
                graphics.draw_mesh(self.mesh, material, transform)
            else:
                graphics.draw_mesh(self.mesh, material, transform, transparency=transparency)



if __name__ == "__main__":
    Shadows().run(AppConfig(title="Oxide Shadows", width=1280, height=800))
