use oxide::{
    app::{
        run,
        AppConfig,
        Game,
        GameTime,
    },
    graphics::{
        Camera,
        Graphics,
        Mesh,
        Vertex,
        PointLight,
        DirectionalLight,
        SpotLight,
        Color,
    },
    math::{
        Angle,
        Transform,
        Vec2,
        Vec3,
    },
};

struct CubeGame {
    cube: Mesh,
    transform: Transform,
    camera: Camera,
    light: SpotLight,
}

impl Game for CubeGame {
    fn initialize(&mut self) {
        println!("Oxide cube example initialized!");
    }

    fn update(&mut self, time: &GameTime) {
        let seconds = time.total_seconds() as f32;

        self.transform.set_pitch(
            Angle::radians(seconds * 0.7)
        );

        self.transform.set_yaw(
            Angle::radians(seconds)
        );

        self.transform.set_roll(
            Angle::radians(seconds * 0.25)
        );
    }

    fn draw(&mut self, graphics: &mut Graphics) {
        let destination =
            graphics.full_rect();

        graphics
            .bind_camera(
                &self.camera,
                destination,
            )
            .expect("failed to bind camera");

        graphics.add_light(
            &self.light
        );

        graphics
            .draw_mesh(
                &self.cube,
                &self.transform,
            )
            .expect("failed to draw cube");
    }
}

fn create_cube() -> Mesh {
    let n_front  = Vec3::new( 0.0,  0.0,  1.0);
    let n_back   = Vec3::new( 0.0,  0.0, -1.0);
    let n_left   = Vec3::new(-1.0,  0.0,  0.0);
    let n_right  = Vec3::new( 1.0,  0.0,  0.0);
    let n_top    = Vec3::new( 0.0,  1.0,  0.0);
    let n_bottom = Vec3::new( 0.0, -1.0,  0.0);

    let uv00 = Vec2::new(0.0, 0.0);
    let uv10 = Vec2::new(1.0, 0.0);
    let uv11 = Vec2::new(1.0, 1.0);
    let uv01 = Vec2::new(0.0, 1.0);

    let vertices = vec![
        // Front (+Z)
        Vertex::new(Vec3::new(-0.5,  0.5,  0.5), n_front, uv00),
        Vertex::new(Vec3::new( 0.5,  0.5,  0.5), n_front, uv10),
        Vertex::new(Vec3::new( 0.5, -0.5,  0.5), n_front, uv11),
        Vertex::new(Vec3::new(-0.5, -0.5,  0.5), n_front, uv01),

        // Back (-Z)
        Vertex::new(Vec3::new( 0.5,  0.5, -0.5), n_back, uv00),
        Vertex::new(Vec3::new(-0.5,  0.5, -0.5), n_back, uv10),
        Vertex::new(Vec3::new(-0.5, -0.5, -0.5), n_back, uv11),
        Vertex::new(Vec3::new( 0.5, -0.5, -0.5), n_back, uv01),

        // Left (-X)
        Vertex::new(Vec3::new(-0.5,  0.5, -0.5), n_left, uv00),
        Vertex::new(Vec3::new(-0.5,  0.5,  0.5), n_left, uv10),
        Vertex::new(Vec3::new(-0.5, -0.5,  0.5), n_left, uv11),
        Vertex::new(Vec3::new(-0.5, -0.5, -0.5), n_left, uv01),

        // Right (+X)
        Vertex::new(Vec3::new( 0.5,  0.5,  0.5), n_right, uv00),
        Vertex::new(Vec3::new( 0.5,  0.5, -0.5), n_right, uv10),
        Vertex::new(Vec3::new( 0.5, -0.5, -0.5), n_right, uv11),
        Vertex::new(Vec3::new( 0.5, -0.5,  0.5), n_right, uv01),

        // Top (+Y)
        Vertex::new(Vec3::new(-0.5, 0.5, -0.5), n_top, uv00),
        Vertex::new(Vec3::new( 0.5, 0.5, -0.5), n_top, uv10),
        Vertex::new(Vec3::new( 0.5, 0.5,  0.5), n_top, uv11),
        Vertex::new(Vec3::new(-0.5, 0.5,  0.5), n_top, uv01),

        // Bottom (-Y)
        Vertex::new(Vec3::new(-0.5, -0.5,  0.5), n_bottom, uv00),
        Vertex::new(Vec3::new( 0.5, -0.5,  0.5), n_bottom, uv10),
        Vertex::new(Vec3::new( 0.5, -0.5, -0.5), n_bottom, uv11),
        Vertex::new(Vec3::new(-0.5, -0.5, -0.5), n_bottom, uv01),
    ];

    let indices = vec![
         0,  1,  2,   0,  2,  3, // Front
         4,  5,  6,   4,  6,  7, // Back
         8,  9, 10,   8, 10, 11, // Left
        12, 13, 14,  12, 14, 15, // Right
        16, 17, 18,  16, 18, 19, // Top
        20, 21, 22,  20, 22, 23, // Bottom
    ];

    Mesh::new(
        vertices,
        indices,
    )
}

fn main() {
    let light = SpotLight::new(
        Vec3::new(
            0.0,
            0.0,
            -1.0,
        ),

        Vec3::new(
            0.0,
            0.0,
            -1.0,
        ),

        Angle::degrees(25.0),

        Color::WHITE,

        15.0,
    );

    let cube = create_cube();

    let mut transform = Transform::new();

    transform.set_position(
        Vec3::new(
            0.0,
            0.0,
            -3.0,
        )
    );

    let camera = Camera::new();

    let game = CubeGame {
        cube,
        transform,
        camera,
        light,
    };

    let config = AppConfig {
        title: String::from("Oxide - 3D Cube"),
        width: 1280,
        height: 720,
    };

    run(game, config);
}