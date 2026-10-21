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
    },
};

struct HelloGame {
    triangle: Mesh,
    elapsed: f64,
}

impl Game for HelloGame {
    fn initialize(&mut self) {
        println!("Oxide initialized!");
    }

    fn update(&mut self, time: &GameTime) {
        self.elapsed += time.delta_seconds();

        // Move the top vertex back and forth.
        self.triangle.vertices_mut()[0].position.x =
            (time.total_seconds() as f32).sin() * 0.5;

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
            .draw_mesh(&self.triangle)
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
        elapsed: 0.0
    };

    let config = AppConfig {
        title: String::from("Oxide"),
        width: 1280,
        height: 720,
    };

    run(game, config);
}