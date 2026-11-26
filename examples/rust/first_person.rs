use oxide::{
    app::{self, AppConfig, Game, GameTime},
    graphics::{
        Camera, Color, DirectionalLight, Graphics, Material, Mesh, PointLight, SpotLight, Vertex,
    },
    input::{Keyboard, Keys, Mouse},
    math::{Angle, Quaternion, Transform, Vec2, Vec3},
};

struct Demo {
    camera: Camera,
    yaw: f32,
    pitch: f32,
    floor: Mesh,
    cube: Mesh,
    green: Material,
    dark: Material,
    brown: Material,
    red: Material,
    directional_lights: [DirectionalLight; 4],
    sun: PointLight,
    flashlight: SpotLight,
    yellow: Material,
    gray: Material,
    captured: bool,
}

impl Demo {
    fn new() -> Self {
        Self {
            camera: Camera::new(),
            yaw: 0.0,
            pitch: 0.0,
            floor: quad(),
            cube: cube(),
            green: Material::solid(Color::rgb(0.25, 0.55, 0.22)),
            dark: Material::solid(Color::rgb(0.12, 0.18, 0.12)),
            brown: Material::solid(Color::rgb(0.45, 0.25, 0.10)),
            red: Material::solid(Color::rgb(0.65, 0.16, 0.10)),
            captured: false,
            directional_lights: [
                DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Color::rgb(1.0, 0.75, 0.65), 0.30).unwrap(),
                DirectionalLight::new(Vec3::new(0.0, 1.0, 0.0), Color::rgb(0.45, 0.55, 1.0), 0.18).unwrap(),
                DirectionalLight::new(Vec3::new(-1.0, 0.0, 0.0), Color::rgb(1.0, 0.35, 0.30), 0.22).unwrap(),
                DirectionalLight::new(Vec3::new(1.0, 0.0, 0.0), Color::rgb(0.30, 1.0, 0.45), 0.22).unwrap(),
            ],
            sun: PointLight::new(Vec3::new(6.0, 7.0, -8.0), Color::rgb(1.0, 0.85, 0.55), 7.0).unwrap(),
            flashlight: SpotLight::new(
                Vec3::new(-4.0, 1.2, -3.0),
                Vec3::new(0.8, -0.15, -1.0),
                Angle::degrees(32.0),
                Color::WHITE,
                5.0,
            ).unwrap(),
            yellow: Material::solid(Color::rgb(1.0, 0.75, 0.10)),
            gray: Material::solid(Color::rgb(0.22, 0.24, 0.28)),
        }
    }

    fn draw_box(
        graphics: &mut Graphics,
        mesh: &Mesh,
        material: &Material,
        position: Vec3,
        scale: Vec3,
    ) {
        let mut transform = Transform::new();
        transform.set_position(position);
        transform.set_scale(scale);
        graphics.draw_mesh(mesh, material, &transform).unwrap();
    }
}

impl Game for Demo {
    fn initialize(&mut self) {
        self.camera.set_position(Vec3::new(0.0, 1.7, 7.0));
        app::set_window_name("Oxide First Person Camera");

    }

    fn update(&mut self, time: &GameTime) {
        let keyboard = Keyboard::get_state();
        let mouse = Mouse::get_state();

        if Mouse::get_state().is_button_pressed(oxide::input::MouseButton::Left)
            && !self.captured
        {
            self.captured = true;
            app::set_cursor_visible(false);
            app::set_cursor_grabbed(true);
        }

        if keyboard.is_key_pressed(Keys::Escape) {
            if self.captured {
                self.captured = false;
                app::set_cursor_grabbed(false);
                app::set_cursor_visible(true);
            } else {
                app::exit();
            }
            return;
        }

        if keyboard.is_key_pressed(Keys::F11) {
            app::set_fullscreen(!app::is_fullscreen());
        }

        if keyboard.is_key_pressed(Keys::F2) {
            app::set_preferred_backbuffer_size(1920, 1080);
        }

        if self.captured {
            self.yaw -= mouse.delta().x * 0.0025;
            self.pitch = (self.pitch - mouse.delta().y * 0.0025).clamp(-1.5, 1.5);
        }

        self.camera.set_euler(
            Angle::radians(self.pitch),
            Angle::radians(self.yaw),
            Angle::ZERO,
        );

        let rotation = Quaternion::from_euler(
            Angle::ZERO,
            Angle::radians(self.yaw),
            Angle::ZERO,
        );

        let forward = rotation.rotate_vector(Vec3::new(0.0, 0.0, -1.0));
        let right = rotation.rotate_vector(Vec3::new(1.0, 0.0, 0.0));

        let mut movement = Vec3::default();
        if keyboard.is_key_down(Keys::W) { movement = movement + forward; }
        if keyboard.is_key_down(Keys::S) { movement = movement - forward; }
        if keyboard.is_key_down(Keys::D) { movement = movement + right; }
        if keyboard.is_key_down(Keys::A) { movement = movement - right; }

        if movement.length() > 0.0 {
            self.camera.translate_by(
                movement.normalized() * (5.0 * time.delta_seconds() as f32)
            );
        }
    }

    fn draw(&mut self, graphics: &mut Graphics) {
        graphics.bind_camera(&self.camera, graphics.full_rect()).unwrap();
        for light in &self.directional_lights {
            graphics.add_light(light);
        }
        graphics.add_light(&self.sun);
        graphics.add_light(&self.flashlight);

        for x in -5..=5 {
            for z in -5..=5 {
                let material = if (x + z) & 1 == 0 {
                    &self.green
                } else {
                    &self.dark
                };

                let mut transform = Transform::new();
                transform.set_position(Vec3::new(x as f32 * 2.0, 0.0, z as f32 * 2.0));
                transform.set_scale(Vec3::new(2.0, 1.0, 2.0));
                graphics.draw_mesh(&self.floor, material, &transform).unwrap();
            }
        }

        // Visible sun marker at exactly the point-light position.
        Self::draw_box(graphics, &self.cube, &self.yellow, Vec3::new(6.0, 7.0, -8.0), Vec3::new(1.2, 1.2, 1.2));

        // Flashlight body. The spotlight originates at its bright front cap
        // and points diagonally across the floor toward the scarecrow.
        Self::draw_box(graphics, &self.cube, &self.gray, Vec3::new(-4.35, 1.25, -2.55), Vec3::new(0.45, 0.45, 1.4));
        Self::draw_box(graphics, &self.cube, &self.yellow, Vec3::new(-4.0, 1.2, -3.0), Vec3::new(0.55, 0.55, 0.18));

        // Scarecrow: post, torso, head, arms, and hat.
        Self::draw_box(graphics, &self.cube, &self.brown, Vec3::new(0.0, 1.5, -6.0), Vec3::new(0.22, 3.0, 0.22));
        Self::draw_box(graphics, &self.cube, &self.red, Vec3::new(0.0, 2.8, -6.0), Vec3::new(1.4, 1.3, 0.55));
        Self::draw_box(graphics, &self.cube, &self.brown, Vec3::new(0.0, 3.9, -6.0), Vec3::new(0.8, 0.8, 0.8));
        Self::draw_box(graphics, &self.cube, &self.brown, Vec3::new(0.0, 3.0, -6.0), Vec3::new(4.0, 0.18, 0.18));
        Self::draw_box(graphics, &self.cube, &self.dark, Vec3::new(0.0, 4.45, -6.0), Vec3::new(1.7, 0.15, 1.2));
        Self::draw_box(graphics, &self.cube, &self.dark, Vec3::new(0.0, 4.75, -6.0), Vec3::new(0.9, 0.55, 0.8));
    }
}

fn quad() -> Mesh {
    Mesh::new(
        vec![
            Vertex::new(Vec3::new(-0.5, 0.0, -0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 0.0)),
            Vertex::new(Vec3::new( 0.5, 0.0, -0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new( 0.5, 0.0,  0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(-0.5, 0.0,  0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 1.0)),
        ],
        vec![0, 1, 2, 0, 2, 3],
    )
}

fn cube() -> Mesh {
    let mut v = Vec::new();
    let mut i = Vec::new();
    let faces = [
        (Vec3::new(0.0,0.0,1.0), [(-1.0,-1.0,1.0),(1.0,-1.0,1.0),(1.0,1.0,1.0),(-1.0,1.0,1.0)]),
        (Vec3::new(0.0,0.0,-1.0), [(1.0,-1.0,-1.0),(-1.0,-1.0,-1.0),(-1.0,1.0,-1.0),(1.0,1.0,-1.0)]),
        (Vec3::new(1.0,0.0,0.0), [(1.0,-1.0,1.0),(1.0,-1.0,-1.0),(1.0,1.0,-1.0),(1.0,1.0,1.0)]),
        (Vec3::new(-1.0,0.0,0.0), [(-1.0,-1.0,-1.0),(-1.0,-1.0,1.0),(-1.0,1.0,1.0),(-1.0,1.0,-1.0)]),
        (Vec3::new(0.0,1.0,0.0), [(-1.0,1.0,1.0),(1.0,1.0,1.0),(1.0,1.0,-1.0),(-1.0,1.0,-1.0)]),
        (Vec3::new(0.0,-1.0,0.0), [(-1.0,-1.0,-1.0),(1.0,-1.0,-1.0),(1.0,-1.0,1.0),(-1.0,-1.0,1.0)]),
    ];
    for (normal, points) in faces {
        let base = v.len() as u32;
        for (x,y,z) in points {
            v.push(Vertex::new(Vec3::new(x*0.5,y*0.5,z*0.5), normal, Vec2::default()));
        }
        i.extend_from_slice(&[base,base+2,base+1,base,base+3,base+2]);
    }
    Mesh::new(v, i)
}

fn main() {
    oxide::app::run(
        Demo::new(),
        AppConfig {
            title: "Oxide First Person Camera".into(),
            width: 1280,
            height: 720,
        },
    );
}
