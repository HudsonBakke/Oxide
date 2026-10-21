use oxide::{
    app::{
        run,
        AppConfig,
        Game,
        GameTime,
    },
    graphics::{
        Graphics,
        Mesh,
        Vertex,
    },
    math::{
        Vec2,
        Vec3,
        Transform,
        Angle,
    },
};

struct HelloGame {
    triangle: Mesh,
    transform: Transform,
    elapsed: f64,
}

impl Game for HelloGame {
    fn initialize(&mut self) {
        println!("Oxide initialized!");
    }

    fn update(&mut self, time: &GameTime) {
        self.elapsed += time.delta_seconds();

        self.transform.set_roll(
            Angle::radians(
                time.total_seconds() as f32
            )
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
        graphics
            .draw_mesh(
                &self.triangle,
                &self.transform,
            )
            .expect("failed to draw triangle");
    }
}

fn main() {
    let game = HelloGame {
        triangle: Mesh::new(
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
        ),
        transform: Transform::new(),
        elapsed: 0.0
    };

    let config = AppConfig {
        title: String::from("Oxide"),
        width: 1280,
        height: 720,
    };

    run(game, config);
}