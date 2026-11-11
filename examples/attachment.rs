use oxide::{
    app::{
        run,
        AppConfig,
        Game,
        GameTime,
    },
    graphics::{
        from_attachment,
        Camera,
        Color,
        DirectionalLight,
        Graphics,
        Material,
        Mesh,
        Vertex,
    },
    math::{
        Angle,
        Transform,
        Vec2,
        Vec3,
    },
};

struct AttachmentGame {
    torso: Mesh,
    arm: Mesh,

    torso_transform: Transform,
    arm_transform: Transform,

    torso_material: Material,
    arm_material: Material,

    camera: Camera,
    light: DirectionalLight,
}

impl Game for AttachmentGame {
    fn initialize(&mut self) {
        println!("Oxide attachment example initialized!");
    }

    fn update(&mut self, time: &GameTime) {
        let seconds =
            time.total_seconds() as f32;

        // Sweep the arm left/right while keeping a
        // substantial downward component.
        let desired_direction =
            Vec3::new(
                seconds.sin(),
                -1.0,
                -0.5,
            )
            .normalized();

        // Also twist the arm around its own longitudinal axis.
        let twist =
            Angle::radians(
                seconds * 0.75
            );

        self.arm_transform =
            from_attachment(
                &self.torso,
                &self.torso_transform,
                Vec3::new(
                    0.75,
                    0.75,
                    0.0,
                ),

                &self.arm,
                Vec3::new(
                    0.0,
                    0.75,
                    0.0,
                ),

                desired_direction,
                twist,
                Vec3::new(1.0, 1.0, 1.0),
            )
            .expect(
                "failed to calculate arm attachment"
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
                &self.torso,
                &self.torso_material,
                &self.torso_transform,
            )
            .expect(
                "failed to draw torso"
            );

        graphics
            .draw_mesh(
                &self.arm,
                &self.arm_material,
                &self.arm_transform,
            )
            .expect(
                "failed to draw arm"
            );
    }
}

fn create_box(
    half_width: f32,
    half_height: f32,
    half_depth: f32,
) -> Mesh {
    let left = -half_width;
    let right = half_width;

    let bottom = -half_height;
    let top = half_height;

    let back = -half_depth;
    let front = half_depth;

    let n_front =
        Vec3::new(0.0, 0.0, 1.0);

    let n_back =
        Vec3::new(0.0, 0.0, -1.0);

    let n_left =
        Vec3::new(-1.0, 0.0, 0.0);

    let n_right =
        Vec3::new(1.0, 0.0, 0.0);

    let n_top =
        Vec3::new(0.0, 1.0, 0.0);

    let n_bottom =
        Vec3::new(0.0, -1.0, 0.0);

    let uv00 = Vec2::new(0.0, 0.0);
    let uv10 = Vec2::new(1.0, 0.0);
    let uv11 = Vec2::new(1.0, 1.0);
    let uv01 = Vec2::new(0.0, 1.0);

    let vertices = vec![
        // Front
        Vertex::new(
            Vec3::new(left, top, front),
            n_front,
            uv00,
        ),
        Vertex::new(
            Vec3::new(right, top, front),
            n_front,
            uv10,
        ),
        Vertex::new(
            Vec3::new(right, bottom, front),
            n_front,
            uv11,
        ),
        Vertex::new(
            Vec3::new(left, bottom, front),
            n_front,
            uv01,
        ),

        // Back
        Vertex::new(
            Vec3::new(right, top, back),
            n_back,
            uv00,
        ),
        Vertex::new(
            Vec3::new(left, top, back),
            n_back,
            uv10,
        ),
        Vertex::new(
            Vec3::new(left, bottom, back),
            n_back,
            uv11,
        ),
        Vertex::new(
            Vec3::new(right, bottom, back),
            n_back,
            uv01,
        ),

        // Left
        Vertex::new(
            Vec3::new(left, top, back),
            n_left,
            uv00,
        ),
        Vertex::new(
            Vec3::new(left, top, front),
            n_left,
            uv10,
        ),
        Vertex::new(
            Vec3::new(left, bottom, front),
            n_left,
            uv11,
        ),
        Vertex::new(
            Vec3::new(left, bottom, back),
            n_left,
            uv01,
        ),

        // Right
        Vertex::new(
            Vec3::new(right, top, front),
            n_right,
            uv00,
        ),
        Vertex::new(
            Vec3::new(right, top, back),
            n_right,
            uv10,
        ),
        Vertex::new(
            Vec3::new(right, bottom, back),
            n_right,
            uv11,
        ),
        Vertex::new(
            Vec3::new(right, bottom, front),
            n_right,
            uv01,
        ),

        // Top
        Vertex::new(
            Vec3::new(left, top, back),
            n_top,
            uv00,
        ),
        Vertex::new(
            Vec3::new(right, top, back),
            n_top,
            uv10,
        ),
        Vertex::new(
            Vec3::new(right, top, front),
            n_top,
            uv11,
        ),
        Vertex::new(
            Vec3::new(left, top, front),
            n_top,
            uv01,
        ),

        // Bottom
        Vertex::new(
            Vec3::new(left, bottom, front),
            n_bottom,
            uv00,
        ),
        Vertex::new(
            Vec3::new(right, bottom, front),
            n_bottom,
            uv10,
        ),
        Vertex::new(
            Vec3::new(right, bottom, back),
            n_bottom,
            uv11,
        ),
        Vertex::new(
            Vec3::new(left, bottom, back),
            n_bottom,
            uv01,
        ),
    ];

    let indices = vec![
         0,  1,  2,   0,  2,  3,
         4,  5,  6,   4,  6,  7,
         8,  9, 10,   8, 10, 11,
        12, 13, 14,  12, 14, 15,
        16, 17, 18,  16, 18, 19,
        20, 21, 22,  20, 22, 23,
    ];

    Mesh::new(
        vertices,
        indices,
    )
}

fn main() {
    let mut torso =
        create_box(
            0.75,
            1.0,
            0.35,
        );

    let mut arm =
        create_box(
            0.30,
            0.75,
            0.10,
        );

    // The arm is modeled vertically, extending downward
    // from its shoulder toward its hand.
    //
    // Its front-facing orientation is +Z.
    arm.set_orientation(
        Vec3::new(
            0.0,
            -1.0,
            0.0,
        ),
        Vec3::new(
            0.0,
            0.0,
            1.0,
        ),
    );

    // Give the torso an intuitive canonical frame as well.
    torso.set_orientation(
        Vec3::new(
            0.0,
            0.0,
            -1.0,
        ),
        Vec3::new(
            0.0,
            1.0,
            0.0,
        ),
    );

    let mut torso_transform =
        Transform::new();

    torso_transform.set_position(
        Vec3::new(
            0.0,
            0.0,
            -5.0,
        )
    );

    let arm_transform =
        from_attachment(
            &torso,
            &torso_transform,
            Vec3::new(
                0.75,
                0.75,
                0.0,
            ),

            &arm,
            Vec3::new(
                0.0,
                0.75,
                0.0,
            ),

            Vec3::new(
                0.0,
                -1.0,
                0.0,
            ),

            Angle::ZERO,
            Vec3::new(1.0, 1.0, 1.0),
        )
        .expect("");

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

    let torso_material =
        Material::solid(
            Color::rgb(
                0.25,
                0.55,
                0.9,
            )
        );

    let arm_material =
        Material::solid(
            Color::rgb(
                0.9,
                0.35,
                0.25,
            )
        );

    let game =
        AttachmentGame {
            torso,
            arm,

            torso_transform,
            arm_transform,

            torso_material,
            arm_material,

            camera,
            light,
        };

    let config =
        AppConfig {
            title:
                String::from(
                    "Oxide - Mesh Attachment"
                ),

            width: 1280,
            height: 720,
        };

    run(game, config);
}