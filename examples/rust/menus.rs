//! Screen-space menu over a 3D scene. Mouse or Up/Down/Enter selects buttons;
//! F flips the sprite, C toggles its clip, Escape exits.
use oxide::{app::{self,AppConfig,Game,GameTime},content::ContentManager,
    graphics::{Camera,Color,DirectionalLight,Draw2DOptions,Environment,Graphics,Material,Mesh,TextOptions,Texture},
    input::{Keyboard,Keys,Mouse,MouseButton},math::{Angle,Rect,Transform,Vec2,Vec3}};
struct Menus {
    white:Texture,atlas:Texture,mesh:Mesh,material:Material,camera:Camera,
    transform:Transform,environment:Environment,light:DirectionalLight,
    selected:usize,paused:bool,exposure:usize,flipped:bool,clipped:bool,angle:f32,
}
impl Menus {
    fn new()->Self {
        let mut camera=Camera::new();camera.set_position(Vec3::new(-1.5,0.0,5.0));
        Self {white:Texture::solid(Color::WHITE),atlas:Texture::rgba(2,2,vec![255,70,70,255,70,255,110,255,70,140,255,255,255,210,70,255]).unwrap(),
            mesh:ContentManager::new().load::<Mesh>("assets/blender_cube.glb").unwrap(),material:Material::solid(Color::rgb(0.1,0.4,0.8)),camera,
            transform:Transform::new(),environment:Environment::new(Color::WHITE,0.2,None,0.0,None,Color::rgb(0.02,0.035,0.065)).unwrap(),
            light:DirectionalLight::new(Vec3::new(-0.5,-1.0,-0.5),Color::WHITE,2.0).unwrap(),
            selected:0,paused:false,exposure:1,flipped:false,clipped:true,angle:0.0}
    }
    fn button(i:usize)->Rect {Rect::new(40.0,130.0+i as f32*68.0,260.0,52.0)}
    fn activate(&mut self) {match self.selected {0=>self.paused=!self.paused,1=>self.exposure=(self.exposure+1)%3,_=>app::exit()}}
    fn panel(&self,g:&mut Graphics,rect:Rect,color:Color) {
        g.draw_2d_with_options(&self.white,rect,Draw2DOptions {tint:color,..Default::default()}).unwrap();
    }
}
impl Game for Menus {
    fn update(&mut self,time:&GameTime) {
        let keys=Keyboard::get_state();let mouse=Mouse::get_state();let p=mouse.position();
        if keys.is_key_pressed(Keys::Up) {self.selected=(self.selected+2)%3;}
        if keys.is_key_pressed(Keys::Down) {self.selected=(self.selected+1)%3;}
        for i in 0..3 {let r=Self::button(i);if p.x>=r.x&&p.x<r.x+r.width&&p.y>=r.y&&p.y<r.y+r.height {
            if mouse.delta().length()>0.0 || mouse.is_button_pressed(MouseButton::Left) {self.selected=i;}
            if mouse.is_button_pressed(MouseButton::Left) {self.activate();}
        }}
        if keys.is_key_pressed(Keys::Enter) {self.activate();}
        if keys.is_key_pressed(Keys::F) {self.flipped=!self.flipped;}
        if keys.is_key_pressed(Keys::C) {self.clipped=!self.clipped;}
        if keys.is_key_pressed(Keys::Escape) {app::exit();}
        if !self.paused {self.angle+=time.delta_seconds() as f32;self.transform.rotate_yaw_by(Angle::radians(time.delta_seconds() as f32));}
    }
    fn draw(&mut self,g:&mut Graphics) {
        g.set_exposure([0.0,1.0,4.0][self.exposure]).unwrap();g.set_environment(&self.environment);g.add_light(&self.light);
        // 2D submission before 3D still appears above the finished 3D scene.
        self.panel(g,Rect::new(24.0,24.0,292.0,348.0),Color::rgba(0.025,0.04,0.08,0.94));
        g.bind_camera(&self.camera,g.full_rect()).unwrap();g.draw_mesh(&self.mesh,&self.material,&self.transform).unwrap();
        let heading=TextOptions {font_size:36.0,..Default::default()};
        g.draw_text_with_options("OXIDE",Vec2::new(40.0,38.0),&heading).unwrap();
        g.draw_text("2D + 3D",Vec2::new(42.0,85.0),Color::rgb(0.45,0.75,1.0)).unwrap();
        let labels=[if self.paused {"Resume".to_string()} else {"Pause".to_string()},format!("Exposure: {}",[0,1,4][self.exposure]),"Quit".into()];
        for (i,label) in labels.iter().enumerate() {
            let r=Self::button(i);self.panel(g,r,if i==self.selected {Color::rgb(0.08,0.3,0.6)} else {Color::rgb(0.06,0.09,0.15)});
            let size=g.measure_text(label).unwrap();g.draw_text(label,Vec2::new(r.x+(r.width-size.x)*0.5,r.y+(r.height-size.y)*0.5),Color::WHITE).unwrap();
        }
        let preview=Rect::new(360.0,48.0,160.0,160.0);
        g.draw_2d_with_options(&self.atlas,preview,Draw2DOptions {rotation:Angle::radians(self.angle*0.4),origin:Vec2::new(80.0,80.0),flip_horizontally:self.flipped,
            clip:if self.clipped {Some(preview)} else {None},tint:Color::rgba(1.0,1.0,1.0,0.8),..Default::default()}).unwrap();
        // One texel crop: no adjacent-cell bleeding when enlarged.
        g.draw_2d_with_options(&self.atlas,Rect::new(360.0,232.0,32.0,32.0),Draw2DOptions {source:Some(Rect::new(1.0,0.0,1.0,1.0)),..Default::default()}).unwrap();
        let help=TextOptions {font_size:16.0,..Default::default()};
        g.draw_text_with_options("Mouse or Up/Down + Enter\nF: flip   C: clip   Escape: quit\nUI ignores scene exposure",Vec2::new(40.0,395.0),&help).unwrap();
    }
}
fn main() -> Result<(), oxide::app::AppError> {app::run(Menus::new(),AppConfig {title:"Oxide Menus".into(),width:900,height:600})}
