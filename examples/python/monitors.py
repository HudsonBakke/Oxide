"""Two automatic Rect3D feeds. Space order; T transparency; B front side; Escape exit."""
import math
from oxide import (AppConfig, Game, GameTime, ContentManager, Mesh, Material, Camera, Color,
                   DirectionalLight, Graphics, Angle, Quaternion, Rect3D, Transform, Vec3, Keyboard, Keys)
from oxide.graphics import Environment


def aim(camera, position, target):
    camera.position=position
    camera.rotation=Quaternion.from_to(Vec3(0,0,-1),target-position)


def monitor(x, front):
    return Rect3D(Vec3(x-1.1,1.4,0),Vec3(x+1.1,1.4,0),Vec3(x+1.1,-0.2,0),Vec3(x-1.1,-0.2,0),Vec3(0,0,1 if front else -1))


class Monitors(Game):
    def __init__(self):
        super().__init__()
        self.mesh=ContentManager().load(Mesh,"assets/blender_cube.glb")
        self.materials=[Material.solid(color) for color in [Color.RED,Color.GREEN,Color.BLUE,Color.rgb(0.35,0.38,0.45),Color.rgb(0.015,0.02,0.03),Color.rgb(1,0.75,0.05)]]
        self.room=[Transform(position=Vec3(10.5,0.6,0),scale=Vec3(0.55,0.55,0.55)),
                   Transform(position=Vec3(12,0.8,-0.5),scale=Vec3(0.5,0.8,0.5)),
                   Transform(position=Vec3(13.5,0.5,0.4),scale=Vec3(0.5,0.5,0.5)),
                   Transform(position=Vec3(12,-0.1,0),scale=Vec3(3.5,0.1,3))]
        self.frames=[Transform(position=Vec3(x,0.6,-0.12),scale=Vec3(1.18,0.88,0.1)) for x in (-1.25,1.25)]
        self.occluder=Transform(position=Vec3(0,0.2,1),scale=Vec3(0.35,0.35,0.35))
        self.screen=Camera()
        aim(self.screen,Vec3(0,1.4,6.5),Vec3(0,0.5,0))
        self.feeds=[Camera(),Camera()]
        aim(self.feeds[0],Vec3(12,2.4,6),Vec3(12,0.5,0))
        aim(self.feeds[1],Vec3(15,4,4.5),Vec3(12,0.5,0))
        self.environment=Environment(Color.WHITE,0.2,None,0,None,Color.rgb(0.035,0.05,0.08))
        self.light=DirectionalLight(Vec3(-0.4,-1,-0.5),Color.WHITE,2.5)
        self.feeds_first=True
        self.front=True
        self.transparency=0.5

    def update(self,time:GameTime):
        t=time.total_seconds
        self.room[0].rotation=Quaternion.from_axis_angle(Vec3(0,1,0),Angle.from_radians(t))
        self.occluder.position=Vec3(math.sin(t*0.7)*2,0.3,1.1)
        aim(self.feeds[0],Vec3(12+math.sin(t)*1.3,2.4,6),Vec3(12,0.5,0))
        keys=Keyboard.get_state()
        if keys.is_key_pressed(Keys.SPACE):
            self.feeds_first=not self.feeds_first
        if keys.is_key_pressed(Keys.T):
            self.transparency=self.transparency+0.5 if self.transparency<1 else 0
        if keys.is_key_pressed(Keys.B):
            self.front=not self.front
        if keys.is_key_pressed(Keys.ESCAPE):
            self.exit()
        self.set_window_name(f"Oxide Monitors | Space order: {'feeds first' if self.feeds_first else 'screen first'} | T transparency: {self.transparency} | B front: {self.front}")

    def draw_room(self,graphics):
        for material,placement in zip(self.materials,self.room):
            graphics.draw_mesh(self.mesh,material,placement)

    def draw_feeds(self,graphics):
        for camera,x in zip(self.feeds,(-1.25,1.25)):
            graphics.bind_camera_3d(camera,monitor(x,self.front))
            self.draw_room(graphics)

    def draw_screen(self,graphics):
        graphics.bind_camera(self.screen,graphics.full_rect)
        self.draw_room(graphics)
        for frame in self.frames:
            graphics.draw_mesh(self.mesh,self.materials[4],frame)
        graphics.draw_mesh(self.mesh,self.materials[5],self.occluder,self.transparency)

    def draw(self,graphics:Graphics):
        graphics.set_environment(self.environment)
        graphics.clear_lights()
        graphics.add_light(self.light)
        if self.feeds_first:
            self.draw_feeds(graphics)
            self.draw_screen(graphics)
        else:
            self.draw_screen(graphics)
            self.draw_feeds(graphics)


if __name__=="__main__":
    Monitors().run(AppConfig(title="Oxide Monitors"))
