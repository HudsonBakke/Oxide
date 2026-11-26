import math

from oxide.app import AppConfig, Game
from oxide.content import ContentManager, Material, Mesh, Texture, Vertex
from oxide.graphics import Camera, Color, DirectionalLight, Environment, Graphics, PointLight
from oxide.math import Transform, Vec2, Vec3

STEPS = 6


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
            vertices.append(Vertex(normal * 0.5, normal, Vec2(u,v)))

    stride = segments + 1
    for ring in range(rings):
        for segment in range(segments):
            a = ring * stride + segment
            b = a + stride
            indices.extend([a,b+1,a+1, a,b,b+1])

    return Mesh(vertices, indices)



def environment_texture() -> Texture:
    width, height = 256, 128
    pixels = bytearray()
    for y in range(height):
        v = y / (height - 1)
        for x in range(width):
            u = x / (width - 1)
            horizon = (1 - abs(v - 0.5) * 2) ** 3
            sun = math.exp(-((u-0.72)**2 + (v-0.32)**2) / 0.0015)
            if v < 0.5:
                r,g,b = 0.12+0.28*horizon, 0.28+0.38*horizon, 0.62+0.28*horizon
            else:
                r,g,b = 0.12+0.20*horizon, 0.10+0.18*horizon, 0.08+0.18*horizon
            pixels.extend((
                int(min(1,r+sun*0.85)*255),
                int(min(1,g+sun*0.72)*255),
                int(min(1,b+sun*0.38)*255),
                255,
            ))
    return Texture.from_rgba(width,height,pixels)


def procedural_normal_map() -> Texture:
    size = 64
    pixels = bytearray()

    for y in range(size):
        for x in range(size):
            fx = x / size * math.tau * 4
            fy = y / size * math.tau * 4
            normal = Vec3(
                -math.cos(fx) * 0.45,
                -math.cos(fy) * 0.45,
                1.0,
            ).normalized()
            pixels.extend((
                int((normal.x * 0.5 + 0.5) * 255),
                int((normal.y * 0.5 + 0.5) * 255),
                int((normal.z * 0.5 + 0.5) * 255),
                255,
            ))

    return Texture.from_rgba(size, size, pixels)


class MaterialTest(Game):
    def __init__(self) -> None:
        super().__init__()
        self.camera = Camera()
        self.sphere = make_sphere(24, 16)
        self.materials: list[Material] = []

        for roughness in range(STEPS):
            for metallic in range(STEPS):
                material = Material.solid(Color.rgb(0.72,0.24,0.12))
                material.metallic = metallic / (STEPS - 1)
                material.roughness = roughness / (STEPS - 1)
                self.materials.append(material)

        self.normal_mapped = Material.solid(Color.rgb(0.32,0.42,0.72))
        self.normal_mapped.roughness = 0.55
        self.normal_mapped.normal_texture = procedural_normal_map()
        self.normal_mapped.normal_strength = 1.0

        self.emissive = Material.solid(Color.rgb(0.08,0.08,0.08))
        self.emissive.emissive_color = Color.rgb(0.1,0.65,1.0)
        self.emissive.emissive_strength = 4.0

        content = ContentManager("materials-hdr-python-cache")
        environment_texture_value = content.load(
            Texture,
            "assets/hdr_background_test.exr",
        )
        self.environment = Environment(
            Color.WHITE,
            0.03,
            environment_texture_value,
            1.0,
            environment_texture_value,
            Color.BLACK,
        )

        self.light = PointLight(Vec3(-2.5,5.5,5), Color.WHITE, 65.0)
        self.fill = DirectionalLight(
            Vec3(0.4,-1,-0.3),
            Color.rgb(0.35,0.42,0.55),
            0.18,
        )

    def initialize(self) -> None:
        self.camera.position = Vec3(0,0,15.5)

    def draw(self, graphics: Graphics) -> None:
        graphics.bind_camera(self.camera, graphics.full_rect)
        graphics.set_environment(self.environment)
        graphics.add_light(self.light)
        graphics.add_light(self.fill)

        spacing = 2.15
        center = (STEPS - 1) * 0.5

        for row in range(STEPS):
            for column in range(STEPS):
                graphics.draw_mesh(
                    self.sphere,
                    self.materials[row * STEPS + column],
                    Transform(
                        position=Vec3(
                            (column - center) * spacing,
                            (center - row) * spacing,
                            0,
                        ),
                        scale=Vec3(1.65,1.65,1.65),
                    ),
                )

        graphics.draw_mesh(
            self.sphere,
            self.normal_mapped,
            Transform(
                position=Vec3(-3,-7,0),
                scale=Vec3(1.8,1.8,1.8),
            ),
        )

        graphics.draw_mesh(
            self.sphere,
            self.emissive,
            Transform(
                position=Vec3(3,-7,0),
                scale=Vec3(1.8,1.8,1.8),
            ),
        )


MaterialTest().run(
    AppConfig(
        title="Oxide Material Test",
        width=1280,
        height=900,
    )
)
