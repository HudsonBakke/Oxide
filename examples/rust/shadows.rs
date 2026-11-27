//! Automatic directional shadows. Space toggles shadows; T cycles box transparency.
use oxide::{
    app::{AppConfig, Game, GameTime},
    graphics::{Camera, Color, DirectionalLight, PointLight, SpotLight, Environment, Graphics, Material, Mesh, Vertex},
    input::{Keyboard, Keys},
    math::{Angle, Quaternion, Transform, Vec2, Vec3},
};

struct Shadows {
    camera: Camera,
    mesh: Mesh,
    materials: [Material; 7],
    transforms: [Transform; 7],
    light: DirectionalLight,
    point: PointLight,
    spot: SpotLight,
    light_mode: usize,
    environment: Environment,
    enabled: bool,
    transparency: f32,
}

impl Shadows {
    fn new() -> Self {
        let mut camera = Camera::new();
        let position = Vec3::new(9.0, 8.0, 13.0);
        camera.set_position(position);
        camera.set_rotation(Quaternion::from_to(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.5, 0.0) - position));
        Self {
            camera,
            mesh: cube(),
            materials: [
                Material::solid(Color::rgb(0.55, 0.58, 0.62)),
                Material::solid(Color::rgb(0.8, 0.18, 0.06)),
                Material::solid(Color::rgb(0.08, 0.35, 0.8)),
                Material::solid(Color::rgb(0.2, 0.8, 0.2)),
                Material::solid(Color::rgb(0.15, 0.7, 0.8)),
                Material::solid(Color::rgb(0.15, 0.7, 0.8)),
                Material::solid(Color::rgb(0.95, 0.75, 0.05)),
            ],
            transforms: [
                instance(Vec3::new(0.0, -0.1, 0.0), Vec3::new(16.0, 0.2, 16.0)),
                instance(Vec3::new(0.0, 1.5, 0.0), Vec3::new(2.0, 2.0, 2.0)),
                instance(Vec3::new(3.0, 1.5, -2.5), Vec3::new(1.5, 3.0, 1.5)),
                instance(Vec3::new(-3.5, 1.0, -3.0), Vec3::new(1.5, 2.0, 1.5)),
                instance(Vec3::new(-3.0, 3.0, 2.0), Vec3::new(2.0, 0.1, 2.0)),
                instance(Vec3::new(-2.5, 4.0, 2.35), Vec3::new(2.0, 0.1, 2.0)),
                instance(Vec3::new(-1.2, 1.0, -1.8), Vec3::new(0.8, 0.8, 0.8)),
            ],
            light: DirectionalLight::new(Vec3::new(-0.5, -1.0, -0.35), Color::WHITE, 3.0).unwrap(),
            point: PointLight::new(Vec3::new(2.0, 7.0, 4.0), Color::WHITE, 100.0).unwrap(),
            spot: SpotLight::new(Vec3::new(0.0, 8.0, 4.0), Vec3::new(0.0, -1.0, -0.5), Angle::degrees(45.0), Color::WHITE, 100.0).unwrap(),
            light_mode: 0,
            environment: Environment::new(Color::WHITE, 0.12, None, 0.0, None, Color::rgb(0.04, 0.06, 0.09)).unwrap(),
            enabled: true,
            transparency: 0.5,
        }
    }
}

impl Game for Shadows {
    fn update(&mut self, time: &GameTime) {
        let t = time.total_seconds() as f32;
        self.transforms[1].set_position(Vec3::new(t.sin() * 2.5, 1.5 + t.cos() * 0.4, 0.0));
        self.transforms[1].set_rotation(Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Angle::radians(t * 0.5)));
        if Keyboard::get_state().is_key_pressed(Keys::Space) {
            self.enabled = !self.enabled;
        }
        if Keyboard::get_state().is_key_pressed(Keys::T) {
            self.transparency = if self.transparency < 1.0 { self.transparency + 0.5 } else { 0.0 };
        }
        if Keyboard::get_state().is_key_pressed(Keys::L) { self.light_mode = (self.light_mode + 1) % 3; }
        oxide::app::set_window_name(format!("Oxide Shadows | {} | box transparency {} | Space shadows, T transparency, L light {}", if self.enabled { "on" } else { "off" }, self.transparency, ["directional", "point", "spot"][self.light_mode]));
    }

    fn draw(&mut self, graphics: &mut Graphics) {
        self.light.set_casts_shadows(self.enabled);
        self.point.set_casts_shadows(self.enabled);
        self.spot.set_casts_shadows(self.enabled);
        graphics.bind_camera(&self.camera, graphics.full_rect()).unwrap();
        graphics.set_environment(&self.environment);
        match self.light_mode {
            0 => graphics.add_light(&self.light),
            1 => graphics.add_light(&self.point),
            _ => graphics.add_light(&self.spot),
        }
        let transmission = [0.0, self.transparency, 0.0, 0.75, 0.5, 0.5, 0.0];
        for ((material, transform), transparency) in self.materials.iter().zip(&self.transforms).zip(transmission) {
            if transparency == 0.0 {
                graphics.draw_mesh(&self.mesh, material, transform).unwrap();
            } else {
                graphics.draw_mesh_with_transparency(&self.mesh, material, transform, transparency).unwrap();
            }
        }
    }
}

fn instance(position: Vec3, scale: Vec3) -> Transform {
    let mut transform = Transform::new();
    transform.set_position(position);
    transform.set_scale(scale);
    transform
}

fn cube() -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    // Each face has separate vertices for its flat normal. Clockwise winding
    // matches the renderer; right and up span the face around its center.
    for (normal, right, up) in [
        (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(0.0, 0.0, -1.0), Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0)),
        (Vec3::new(0.0, -1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
    ] {
        let base = vertices.len() as u32;
        for (x, y) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            vertices.push(Vertex::new((normal + right * x + up * y) * 0.5, normal, Vec2::new((x + 1.0) * 0.5, (y + 1.0) * 0.5)));
        }
        indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
    }
    Mesh::new(vertices, indices)
}

fn main() {
    oxide::app::run(Shadows::new(), AppConfig { title: "Oxide Shadows".into(), width: 1280, height: 800 });
}
