use oxide::{
    collision::collides,
    content::ContentManager,
    graphics::Mesh,
    math::{
        Transform,
        Vec3,
    },
};

#[test]
fn collides_handles_recursive_bvh() {
    let mut content =
        ContentManager::new();

    let mesh_a =
        content
            .load::<Mesh>(
                "assets/blender_cube.glb"
            )
            .expect(
                "first cube should load"
            );

    let mesh_b =
        content
            .load::<Mesh>(
                "assets/blender_cube.glb"
            )
            .expect(
                "second cube should load"
            );

    let transform_a =
        Transform::new();

    let transform_b =
        Transform::new();

    assert!(
        collides(
            &mesh_a,
            &transform_a,
            &mesh_b,
            &transform_b,
        ),
        "overlapping cubes should collide"
    );

    let mut separated =
        Transform::new();

    separated.set_position(
        Vec3::new(
            100.0,
            0.0,
            0.0,
        )
    );

    assert!(
        !collides(
            &mesh_a,
            &transform_a,
            &mesh_b,
            &separated,
        ),
        "widely separated cubes should not collide"
    );
}