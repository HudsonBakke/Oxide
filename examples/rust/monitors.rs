//! Two automatic Rect3D camera feeds. Space changes submission order, T changes
//! occluder transparency, B flips the monitor fronts, Escape exits.
use oxide::{app::{self, AppConfig, Game, GameTime}, content::ContentManager,
    graphics::{Camera, Color, DirectionalLight, Environment, Graphics, Material, Mesh},
    input::{Keyboard, Keys}, math::{Angle, Quaternion, Rect3D, Transform, Vec3}};

fn placement(position: Vec3, scale: Vec3) -> Transform {
    let mut t=Transform::new();t.set_position(position);t.set_scale(scale);t
}
fn aim(camera: &mut Camera, position: Vec3, target: Vec3) {
    camera.set_position(position);camera.set_rotation(Quaternion::from_to(Vec3::new(0.0,0.0,-1.0),target-position));
}
fn monitor(center: f32, front: bool) -> Rect3D {
    Rect3D::new(Vec3::new(center-1.1,1.4,0.0),Vec3::new(center+1.1,1.4,0.0),
        Vec3::new(center+1.1,-0.2,0.0),Vec3::new(center-1.1,-0.2,0.0),Vec3::new(0.0,0.0,if front {1.0} else {-1.0})).unwrap()
}
struct Monitors {
    mesh: Mesh, materials: [Material;6], room: [Transform;4], frames: [Transform;2],
    occluder: Transform, screen: Camera, feeds: [Camera;2], environment: Environment,
    light: DirectionalLight, feeds_first: bool, front: bool, transparency: f32,
}
impl Monitors {
    fn new() -> Self {
        let mesh=ContentManager::new().load::<Mesh>("assets/blender_cube.glb").expect("load cube");
        let mut screen=Camera::new();aim(&mut screen,Vec3::new(0.0,1.4,6.5),Vec3::new(0.0,0.5,0.0));
        let mut feeds=[Camera::new();2];aim(&mut feeds[0],Vec3::new(12.0,2.4,6.0),Vec3::new(12.0,0.5,0.0));
        aim(&mut feeds[1],Vec3::new(15.0,4.0,4.5),Vec3::new(12.0,0.5,0.0));
        Self { mesh, materials:[Material::solid(Color::RED),Material::solid(Color::GREEN),Material::solid(Color::BLUE),
            Material::solid(Color::rgb(0.35,0.38,0.45)),Material::solid(Color::rgb(0.015,0.02,0.03)),Material::solid(Color::rgb(1.0,0.75,0.05))],
            room:[placement(Vec3::new(10.5,0.6,0.0),Vec3::new(0.55,0.55,0.55)),placement(Vec3::new(12.0,0.8,-0.5),Vec3::new(0.5,0.8,0.5)),
                placement(Vec3::new(13.5,0.5,0.4),Vec3::new(0.5,0.5,0.5)),placement(Vec3::new(12.0,-0.1,0.0),Vec3::new(3.5,0.1,3.0))],
            frames:[placement(Vec3::new(-1.25,0.6,-0.12),Vec3::new(1.18,0.88,0.1)),placement(Vec3::new(1.25,0.6,-0.12),Vec3::new(1.18,0.88,0.1))],
            occluder:placement(Vec3::new(0.0,0.2,1.0),Vec3::new(0.35,0.35,0.35)), screen, feeds,
            environment:Environment::new(Color::WHITE,0.2,None,0.0,None,Color::rgb(0.035,0.05,0.08)).unwrap(),
            light:DirectionalLight::new(Vec3::new(-0.4,-1.0,-0.5),Color::WHITE,2.5).unwrap(),
            feeds_first:true,front:true,transparency:0.5 }
    }
    fn draw_room(&self, graphics: &mut Graphics) {
        for i in 0..4 { graphics.draw_mesh(&self.mesh,&self.materials[i],&self.room[i]).unwrap(); }
    }
    fn draw_feeds(&self, graphics: &mut Graphics) {
        for (index,x) in [-1.25,1.25].into_iter().enumerate() {
            graphics.bind_camera_3d(&self.feeds[index],monitor(x,self.front)).unwrap();
            self.draw_room(graphics);
        }
    }
    fn draw_screen(&self, graphics: &mut Graphics) {
        graphics.bind_camera(&self.screen,graphics.full_rect()).unwrap();self.draw_room(graphics);
        for frame in &self.frames { graphics.draw_mesh(&self.mesh,&self.materials[4],frame).unwrap(); }
        graphics.draw_mesh_with_transparency(&self.mesh,&self.materials[5],&self.occluder,self.transparency).unwrap();
    }
}
impl Game for Monitors {
    fn update(&mut self,time: &GameTime) {
        let t=time.total_seconds() as f32;
        self.room[0].set_rotation(Quaternion::from_axis_angle(Vec3::new(0.0,1.0,0.0),Angle::radians(t)));
        self.occluder.set_position(Vec3::new((t*0.7).sin()*2.0,0.3,1.1));
        aim(&mut self.feeds[0],Vec3::new(12.0+t.sin()*1.3,2.4,6.0),Vec3::new(12.0,0.5,0.0));
        let keys=Keyboard::get_state();
        if keys.is_key_pressed(Keys::Space) { self.feeds_first=!self.feeds_first; }
        if keys.is_key_pressed(Keys::T) { self.transparency=if self.transparency<1.0 {self.transparency+0.5} else {0.0}; }
        if keys.is_key_pressed(Keys::B) { self.front=!self.front; }
        if keys.is_key_pressed(Keys::Escape) { app::exit(); }
        app::set_window_name(format!("Oxide Monitors | Space order: {} | T transparency: {} | B front: {}",if self.feeds_first {"feeds first"} else {"screen first"},self.transparency,self.front));
    }
    fn draw(&mut self, graphics: &mut Graphics) {
        graphics.set_environment(&self.environment);graphics.clear_lights();graphics.add_light(&self.light);
        if self.feeds_first { self.draw_feeds(graphics);self.draw_screen(graphics); }
        else { self.draw_screen(graphics);self.draw_feeds(graphics); }
    }
}
fn main() -> Result<(), oxide::app::AppError> { app::run(Monitors::new(),AppConfig { title:"Oxide Monitors".into(),..Default::default() }) }
