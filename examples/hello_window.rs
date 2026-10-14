use oxide::app::{run, AppConfig, Game, GameTime};

struct HelloGame {
    elapsed: f64,
}

impl Game for HelloGame {
    fn initialize(&mut self) {
        println!("Oxide initialized!");
    }

    fn update(&mut self, time: &GameTime) {
        self.elapsed += time.delta_seconds();

        if self.elapsed >= 1.0 {
            println!(
                "Oxide has been running for {:.2} seconds",
                time.total_seconds()
            );

            self.elapsed = 0.0;
        }
    }
}

fn main() {
    let game = HelloGame {
        elapsed: 0.0
    };

    let config = AppConfig {
        title: String::from("Oxide"),
        width: 1280,
        height: 720,
    };

    run(game, config);
}