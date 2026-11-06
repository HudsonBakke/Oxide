use std::hint::black_box;

use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};

use oxide::{
    collision,
    graphics::{
        Mesh,
        Vertex,
    },
    math::{
        Transform,
        Vec2,
        Vec3,
    },
};

fn triangle_mesh(
    triangle_count: usize,
    spacing: f32,
) -> Mesh {
    let mut vertices =
        Vec::with_capacity(triangle_count * 3);

    let mut indices =
        Vec::with_capacity(triangle_count * 3);

    for i in 0..triangle_count {
        let x =
            (i as f32 * spacing) % 100.0;

        let y =
            ((i / 100) as f32 * spacing)
                % 100.0;

        let z =
            (i / 10_000) as f32 * spacing;

        let base =
            vertices.len() as u32;

        vertices.push(
            Vertex::new(
                Vec3::new(
                    x,
                    y,
                    z,
                ),
                Vec3::new(
                    0.0,
                    0.0,
                    1.0,
                ),
                Vec2::default(),
            )
        );

        vertices.push(
            Vertex::new(
                Vec3::new(
                    x + 0.5,
                    y,
                    z,
                ),
                Vec3::new(
                    0.0,
                    0.0,
                    1.0,
                ),
                Vec2::default(),
            )
        );

        vertices.push(
            Vertex::new(
                Vec3::new(
                    x,
                    y + 0.5,
                    z,
                ),
                Vec3::new(
                    0.0,
                    0.0,
                    1.0,
                ),
                Vec2::default(),
            )
        );

        indices.extend_from_slice(&[
            base,
            base + 1,
            base + 2,
        ]);
    }

    Mesh::new(
        vertices,
        indices,
    )
}

fn cube_mesh() -> Mesh {
    let positions = [
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new( 0.5, -0.5, -0.5),
        Vec3::new( 0.5,  0.5, -0.5),
        Vec3::new(-0.5,  0.5, -0.5),

        Vec3::new(-0.5, -0.5,  0.5),
        Vec3::new( 0.5, -0.5,  0.5),
        Vec3::new( 0.5,  0.5,  0.5),
        Vec3::new(-0.5,  0.5,  0.5),
    ];

    let vertices = positions
        .into_iter()
        .map(|position| {
            Vertex::new(
                position,
                Vec3::default(),
                Vec2::default(),
            )
        })
        .collect();

    let indices = vec![
        0, 2, 1,
        0, 3, 2,

        4, 5, 6,
        4, 6, 7,

        0, 4, 7,
        0, 7, 3,

        1, 2, 6,
        1, 6, 5,

        3, 7, 6,
        3, 6, 2,

        0, 1, 5,
        0, 5, 4,
    ];

    Mesh::new(
        vertices,
        indices,
    )
}

fn benchmark_hitboxes(
    criterion: &mut Criterion,
) {
    let mesh_a = cube_mesh();
    let mesh_b = cube_mesh();

    let transform_a =
        Transform::new();

    let mut transform_b =
        Transform::new();

    transform_b.set_position(
        Vec3::new(
            0.75,
            0.0,
            0.0,
        )
    );

    // Warm the BVH cache before measuring.
    black_box(
        collision::collides(
            &mesh_a,
            &transform_a,
            &mesh_b,
            &transform_b,
        )
    );

    criterion.bench_function(
        "collision/simple_hitboxes",
        |bencher| {
            bencher.iter(|| {
                black_box(
                    collision::collides(
                        black_box(&mesh_a),
                        black_box(&transform_a),
                        black_box(&mesh_b),
                        black_box(&transform_b),
                    )
                )
            });
        },
    );
}

fn benchmark_separated_meshes(
    criterion: &mut Criterion,
) {
    let mut group =
        criterion.benchmark_group(
            "collision/separated"
        );

    for triangle_count in [
        100usize,
        1_000,
        10_000,
        100_000,
    ] {
        let mesh_a =
            triangle_mesh(
                triangle_count,
                1.0,
            );

        let mesh_b =
            triangle_mesh(
                triangle_count,
                1.0,
            );

        let transform_a =
            Transform::new();

        let mut transform_b =
            Transform::new();

        transform_b.set_position(
            Vec3::new(
                10_000.0,
                0.0,
                0.0,
            )
        );

        // Build/cache BVHs before measurement.
        black_box(
            collision::collides(
                &mesh_a,
                &transform_a,
                &mesh_b,
                &transform_b,
            )
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(
                triangle_count
            ),
            &triangle_count,
            |bencher, _| {
                bencher.iter(|| {
                    black_box(
                        collision::collides(
                            black_box(&mesh_a),
                            black_box(&transform_a),
                            black_box(&mesh_b),
                            black_box(&transform_b),
                        )
                    )
                });
            },
        );
    }

    group.finish();
}

fn benchmark_intersecting_meshes(
    criterion: &mut Criterion,
) {
    let mut group =
        criterion.benchmark_group(
            "collision/intersecting"
        );

    for triangle_count in [
        100usize,
        1_000,
        10_000,
    ] {
        let mesh_a =
            triangle_mesh(
                triangle_count,
                1.0,
            );

        let mesh_b =
            triangle_mesh(
                triangle_count,
                1.0,
            );

        let transform_a =
            Transform::new();

        let transform_b =
            Transform::new();

        black_box(
            collision::collides(
                &mesh_a,
                &transform_a,
                &mesh_b,
                &transform_b,
            )
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(
                triangle_count
            ),
            &triangle_count,
            |bencher, _| {
                bencher.iter(|| {
                    black_box(
                        collision::collides(
                            black_box(&mesh_a),
                            black_box(&transform_a),
                            black_box(&mesh_b),
                            black_box(&transform_b),
                        )
                    )
                });
            },
        );
    }

    group.finish();
}

fn benchmark_near_miss(
    criterion: &mut Criterion,
) {
    let mesh_a =
        cube_mesh();

    let mesh_b =
        cube_mesh();

    let transform_a =
        Transform::new();

    let mut transform_b =
        Transform::new();

    // Close, but not touching.
    transform_b.set_position(
        Vec3::new(
            1.001,
            0.0,
            0.0,
        )
    );

    black_box(
        collision::collides(
            &mesh_a,
            &transform_a,
            &mesh_b,
            &transform_b,
        )
    );

    criterion.bench_function(
        "collision/near_miss",
        |bencher| {
            bencher.iter(|| {
                black_box(
                    collision::collides(
                        black_box(&mesh_a),
                        black_box(&transform_a),
                        black_box(&mesh_b),
                        black_box(&transform_b),
                    )
                )
            });
        },
    );
}

criterion_group!(
    benches,
    benchmark_hitboxes,
    benchmark_separated_meshes,
    benchmark_intersecting_meshes,
    benchmark_near_miss,
);

criterion_main!(benches);