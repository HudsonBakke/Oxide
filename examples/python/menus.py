"""2D menu over 3D. Mouse or Up/Down/Enter selects; F flips, C clips, Escape exits."""
from oxide import (AppConfig, Game, ContentManager, Mesh, Material, Texture, Camera,
                   Color, DirectionalLight, Draw2DOptions, TextOptions, Angle, Rect,
                   Transform, Vec2, Vec3, Keyboard, Keys, Mouse, MouseButtons)
from oxide.graphics import Environment


class Menus(Game):
    def __init__(self):
        super().__init__()
        self.white = Texture.solid(Color.WHITE)
        self.atlas = Texture.from_rgba(2, 2, bytes([255,70,70,255, 70,255,110,255, 70,140,255,255, 255,210,70,255]))
        self.mesh = ContentManager().load(Mesh, "assets/blender_cube.glb")
        self.material = Material.solid(Color.rgb(0.1, 0.4, 0.8))
        self.camera = Camera()
        self.camera.position = Vec3(-1.5, 0, 5)
        self.transform = Transform()
        self.environment = Environment(Color.WHITE, 0.2, None, 0, None, Color.rgb(0.02, 0.035, 0.065))
        self.light = DirectionalLight(Vec3(-0.5, -1, -0.5), Color.WHITE, 2)
        self.selected, self.exposure = 0, 1
        self.paused, self.flipped, self.clipped = False, False, True
        self.angle = 0.0

    @staticmethod
    def button(index):
        return Rect(40, 130 + index * 68, 260, 52)

    def activate(self):
        if self.selected == 0:
            self.paused = not self.paused
        elif self.selected == 1:
            self.exposure = (self.exposure + 1) % 3
        else:
            self.exit()

    def update(self, time):
        keys, mouse = Keyboard.get_state(), Mouse.get_state()
        if keys.is_key_pressed(Keys.UP):
            self.selected = (self.selected + 2) % 3
        if keys.is_key_pressed(Keys.DOWN):
            self.selected = (self.selected + 1) % 3
        p = mouse.position
        for i in range(3):
            r = self.button(i)
            if r.x <= p.x < r.x + r.width and r.y <= p.y < r.y + r.height:
                if mouse.delta.length > 0 or mouse.is_button_pressed(MouseButtons.LEFT):
                    self.selected = i
                if mouse.is_button_pressed(MouseButtons.LEFT):
                    self.activate()
        if keys.is_key_pressed(Keys.ENTER):
            self.activate()
        if keys.is_key_pressed(Keys.F):
            self.flipped = not self.flipped
        if keys.is_key_pressed(Keys.C):
            self.clipped = not self.clipped
        if keys.is_key_pressed(Keys.ESCAPE):
            self.exit()
        if not self.paused:
            self.angle += time.delta_seconds
            self.transform.rotate_yaw_by(Angle.from_radians(time.delta_seconds))

    def panel(self, graphics, rect, color):
        graphics.draw_2d(self.white, rect, Draw2DOptions(tint=color))

    def draw(self, graphics):
        graphics.exposure = (0, 1, 4)[self.exposure]
        graphics.set_environment(self.environment)
        graphics.add_light(self.light)
        # Submission before the mesh still produces an overlay above it.
        self.panel(graphics, Rect(24,24,292,348), Color(0.025,0.04,0.08,0.94))
        graphics.bind_camera(self.camera, graphics.full_rect)
        graphics.draw_mesh(self.mesh, self.material, self.transform)
        graphics.draw_text("OXIDE", Vec2(40,38), options=TextOptions(font_size=36))
        graphics.draw_text("2D + 3D", Vec2(42,85), Color(0.45,0.75,1))
        labels = ["Resume" if self.paused else "Pause", f"Exposure: {(0,1,4)[self.exposure]}", "Quit"]
        for i, label in enumerate(labels):
            r = self.button(i)
            self.panel(graphics, r, Color(0.08,0.3,0.6) if i == self.selected else Color(0.06,0.09,0.15))
            size = graphics.measure_text(label)
            graphics.draw_text(label, Vec2(r.x+(r.width-size.x)*0.5, r.y+(r.height-size.y)*0.5))
        preview = Rect(360,48,160,160)
        graphics.draw_2d(self.atlas, preview, Draw2DOptions(rotation=Angle.from_radians(self.angle*0.4), origin=Vec2(80,80),
            flip_horizontally=self.flipped, clip=preview if self.clipped else None, tint=Color(1,1,1,0.8)))
        graphics.draw_2d(self.atlas, Rect(360,232,32,32), Draw2DOptions(source=Rect(1,0,1,1)))
        graphics.draw_text("Mouse or Up/Down + Enter\nF: flip   C: clip   Escape: quit\nUI ignores scene exposure", Vec2(40,395), options=TextOptions(font_size=16))


if __name__ == "__main__":
    Menus().run(AppConfig(title="Oxide Menus", width=900, height=600))
