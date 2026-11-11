use oxide::{
    app::{
        run,
        AppConfig,
        Game,
        GameTime,
    },
    content::ContentManager,
    graphics::{
        Camera,
        Color,
        DirectionalLight,
        Graphics,
        Material,
        Mesh,
    },
    math::{
        Angle,
        Transform,
        Vec3,
    },
};

struct ModelGame {
    mesh: Mesh,
    transform: Transform,
    camera: Camera,
    light: DirectionalLight,
    material: Material,
}

impl Game for ModelGame {
    fn initialize(&mut self) {
        println!("Oxide model example initialized!");
    }

    fn update(&mut self, time: &GameTime) {
        let seconds =
            time.total_seconds() as f32;

        self.transform.set_euler(
            Angle::radians(
                seconds * 0.35
            ),
            Angle::radians(
                seconds * 0.7
            ),
            Angle::ZERO,
        );
    }

    fn draw(
        &mut self,
        graphics: &mut Graphics,
    ) {
        graphics
            .bind_camera(
                &self.camera,
                graphics.full_rect(),
            )
            .expect(
                "failed to bind camera"
            );

        graphics.add_light(
            &self.light
        );

        graphics
            .draw_mesh(
                &self.mesh,
                &self.material,
                &self.transform,
            )
            .expect(
                "failed to draw model"
            );
    }
}

fn main() {
    let mut content =
        ContentManager::new();

    // This is the important part of the example:
    // the mesh comes directly from the Blender-exported GLB.
    let mesh =
        content
            .load::<Mesh>(
                "assets/textured_material_cube.glb"
            )
            .expect(
                "failed to load Blender mesh"
            );

    let material =
        content
            .load::<Material>(
                "assets/textured_material_cube.glb"
            )
            .expect(
                "failed to load Blender material"
            );

    let mut transform =
        Transform::new();

    transform.set_position(
        Vec3::new(
            0.0,
            0.0,
            -5.0,
        )
    );

    let camera =
        Camera::new();

    let light =
        DirectionalLight::new(
            Vec3::new(
                1.0,
                -1.0,
                -1.0,
            ),
            Color::WHITE,
            1.0,
        );

    let game =
        ModelGame {
            mesh,
            transform,
            camera,
            light,
            material,
        };

    let config =
        AppConfig {
            title:
                String::from(
                    "Oxide - Imported Model"
                ),

            width: 1280,
            height: 720,
        };

    run(game, config);
}