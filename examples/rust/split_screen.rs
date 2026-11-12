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
        Material,
        Color,
        Mesh,
        Vertex,
    },
    math::{
        Angle,
        Rect,
        Transform,
        Vec2,
        Vec3,
    },
};

struct SplitScreenGame {
    triangle: Mesh,

    left_transform: Transform,
    right_transform: Transform,

    left_camera: Camera,
    right_camera: Camera,

    elapsed: f64,
    material: Material,
}

impl Game for SplitScreenGame {
    fn initialize(&mut self) {
        println!("Oxide split-screen example initialized!");
    }

    fn update(&mut self, time: &GameTime) {
        self.elapsed += time.delta_seconds();

        // Slowly rotate the right-hand triangle so it is obvious
        // that both halves of the screen are updating normally.
        self.right_transform.set_euler(
            Angle::ZERO,
            Angle::ZERO,
            Angle::radians(
                time.total_seconds() as f32
            ),
        );

        if self.elapsed >= 1.0 {
            println!(
                "Oxide has been running for {:.2} seconds",
                time.total_seconds()
            );

            self.elapsed = 0.0;
        }
    }

    fn draw(&mut self, graphics: &mut Graphics) {
        let screen = graphics.full_rect();

        let half_width = screen.width / 2.0;

        let left_rect = Rect::new(
            screen.x,
            screen.y,
            half_width,
            screen.height,
        );

        let right_rect = Rect::new(
            screen.x + half_width,
            screen.y,
            half_width,
            screen.height,
        );

        // Left camera.
        graphics
            .bind_camera(
                &self.left_camera,
                left_rect,
            )
            .expect("failed to bind left camera");

        graphics
            .draw_mesh(
                &self.triangle,
                &self.material,
                &self.left_transform,
            )
            .expect("failed to draw from left camera");

        graphics
            .draw_mesh(
                &self.triangle,
                &self.material,
                &self.right_transform,
            )
            .expect("failed to draw from left camera");

        // Right camera.
        graphics
            .bind_camera(
                &self.right_camera,
                right_rect,
            )
            .expect("failed to bind right camera");

        graphics
            .draw_mesh(
                &self.triangle,
                &self.material,
                &self.left_transform,
            )
            .expect("failed to draw from right camera");

        graphics
            .draw_mesh(
                &self.triangle,
                &self.material,
                &self.right_transform,
            )
            .expect("failed to draw from right camera");
    }
}

fn main() {
    let material =
        Material::solid(Color::WHITE);

    let triangle = Mesh::new(
        vec![
            Vertex::new(
                Vec3::new(0.0, 0.5, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec2::new(0.5, 0.0),
            ),
            Vertex::new(
                Vec3::new(0.5, -0.5, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec2::new(1.0, 1.0),
            ),
            Vertex::new(
                Vec3::new(-0.5, -0.5, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec2::new(0.0, 1.0),
            ),
        ],
        vec![0, 1, 2],
    );

    // Put two instances of the same mesh into the world.
    let mut left_transform = Transform::new();

    left_transform.set_position(
        Vec3::new(
            -0.75,
            0.0,
            -4.0,
        )
    );

    let mut right_transform = Transform::new();

    right_transform.set_position(
        Vec3::new(
            0.75,
            0.0,
            -4.0,
        )
    );

    // Left camera stays at the world origin.
    let left_camera = Camera::new();

    // Right camera is shifted to the right and turned slightly
    // toward the scene.
    let mut right_camera = Camera::new();

    right_camera.set_position(
        Vec3::new(
            1.5,
            0.0,
            0.0,
        )
    );

    right_camera.set_euler(
        Angle::ZERO,
        Angle::degrees(-15.0),
        Angle::ZERO,
    );

    let game = SplitScreenGame {
        triangle,

        left_transform,
        right_transform,

        left_camera,
        right_camera,

        elapsed: 0.0,
        material: material,
    };

    let config = AppConfig {
        title: String::from(
            "Oxide - Split Screen"
        ),
        width: 1280,
        height: 720,
    };

    run(game, config);
}