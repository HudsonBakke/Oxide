use oxide::{
    app::{AppConfig, Game},
    graphics::{Camera, Color, DirectionalLight, Graphics, Material, Mesh, PointLight, Texture, Vertex},
    math::{Transform, Vec2, Vec3},
};

const STEPS: usize = 6;

struct MaterialTest {
    camera: Camera,
    sphere: Mesh,
    materials: Vec<Material>,
    emissive: Material,
    normal_mapped: Material,
    light: PointLight,
    fill: DirectionalLight,
}

impl MaterialTest {
    fn new() -> Self {
        let mut materials = Vec::with_capacity(STEPS * STEPS);

        for roughness_index in 0..STEPS {
            for metallic_index in 0..STEPS {
                let mut material = Material::solid(Color::rgb(0.72, 0.24, 0.12));
                material
                    .set_metallic(metallic_index as f32 / (STEPS - 1) as f32)
                    .unwrap();
                material
                    .set_roughness(roughness_index as f32 / (STEPS - 1) as f32)
                    .unwrap();
                materials.push(material);
            }
        }

        let mut normal_mapped = Material::solid(Color::rgb(0.32, 0.42, 0.72));
        normal_mapped.set_roughness(0.55).unwrap();
        normal_mapped.set_normal_texture(procedural_normal_map());
        normal_mapped.set_normal_strength(1.0).unwrap();

        let mut emissive = Material::solid(Color::rgb(0.08, 0.08, 0.08));
        emissive.set_emissive_color(Color::rgb(0.1, 0.65, 1.0));
        emissive.set_emissive_strength(4.0).unwrap();

        Self {
            camera: Camera::new(),
            sphere: sphere(24, 16),
            materials,
            emissive,
            normal_mapped,
            light: PointLight::new(
                Vec3::new(-2.5, 5.5, 5.0),
                Color::WHITE,
                65.0,
            )
            .unwrap(),
            fill: DirectionalLight::new(
                Vec3::new(0.4, -1.0, -0.3),
                Color::rgb(0.35, 0.42, 0.55),
                0.18,
            )
            .unwrap(),
        }
    }
}

impl Game for MaterialTest {
    fn initialize(&mut self) {
        self.camera.set_position(Vec3::new(0.0, 0.0, 15.5));
    }

    fn draw(&mut self, graphics: &mut Graphics) {
        graphics.bind_camera(&self.camera, graphics.full_rect()).unwrap();
        graphics.add_light(&self.light);
        graphics.add_light(&self.fill);

        let spacing = 2.15;
        let center = (STEPS as f32 - 1.0) * 0.5;

        for row in 0..STEPS {
            for column in 0..STEPS {
                let mut transform = Transform::new();
                transform.set_position(Vec3::new(
                    (column as f32 - center) * spacing,
                    (center - row as f32) * spacing,
                    0.0,
                ));
                transform.set_scale(Vec3::new(1.65, 1.65, 1.65));

                graphics
                    .draw_mesh(
                        &self.sphere,
                        &self.materials[row * STEPS + column],
                        &transform,
                    )
                    .unwrap();
            }
        }

        let mut normal_transform = Transform::new();
        normal_transform.set_position(Vec3::new(-3.0, -7.0, 0.0));
        normal_transform.set_scale(Vec3::new(1.8, 1.8, 1.8));
        graphics
            .draw_mesh(&self.sphere, &self.normal_mapped, &normal_transform)
            .unwrap();

        let mut transform = Transform::new();
        transform.set_position(Vec3::new(3.0, -7.0, 0.0));
        transform.set_scale(Vec3::new(1.8, 1.8, 1.8));
        graphics
            .draw_mesh(&self.sphere, &self.emissive, &transform)
            .unwrap();
    }
}


fn procedural_normal_map() -> Texture {
    let size = 64_u32;
    let mut pixels = Vec::with_capacity((size * size * 4) as usize);

    for y in 0..size {
        for x in 0..size {
            let fx = x as f32 / size as f32 * std::f32::consts::TAU * 4.0;
            let fy = y as f32 / size as f32 * std::f32::consts::TAU * 4.0;
            let dx = fx.cos() * 0.45;
            let dy = fy.cos() * 0.45;
            let normal = Vec3::new(-dx, -dy, 1.0).normalized();

            pixels.extend_from_slice(&[
                ((normal.x * 0.5 + 0.5) * 255.0) as u8,
                ((normal.y * 0.5 + 0.5) * 255.0) as u8,
                ((normal.z * 0.5 + 0.5) * 255.0) as u8,
                255,
            ]);
        }
    }

    Texture::rgba(size, size, pixels).unwrap()
}

fn sphere(segments: u32, rings: u32) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for ring in 0..=rings {
        let v = ring as f32 / rings as f32;
        let phi = v * std::f32::consts::PI;
        let y = phi.cos();
        let radius = phi.sin();

        for segment in 0..=segments {
            let u = segment as f32 / segments as f32;
            let theta = u * std::f32::consts::TAU;
            let normal = Vec3::new(
                radius * theta.cos(),
                y,
                radius * theta.sin(),
            );

            vertices.push(Vertex::new(
                normal * 0.5,
                normal,
                Vec2::new(u, v),
            ));
        }
    }

    let stride = segments + 1;
    for ring in 0..rings {
        for segment in 0..segments {
            let a = ring * stride + segment;
            let b = a + stride;
            indices.extend_from_slice(&[
                a, b + 1, a + 1,
                a, b, b + 1,
            ]);
        }
    }

    Mesh::new(vertices, indices)
}

fn main() {
    oxide::app::run(
        MaterialTest::new(),
        AppConfig {
            title: "Oxide Material Test".into(),
            width: 1280,
            height: 900,
        },
    );
}
