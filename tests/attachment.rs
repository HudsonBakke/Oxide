use oxide::{
    graphics::{
        from_attachment,
        Mesh,
    },
    math::{
        Angle,
        Transform,
        Vec3,
    },
};

const EPSILON: f32 = 0.0001;

fn assert_vec3_close(
    actual: Vec3,
    expected: Vec3,
) {
    assert!(
        (actual.x - expected.x).abs() < EPSILON,
        "x: expected {}, got {}",
        expected.x,
        actual.x,
    );

    assert!(
        (actual.y - expected.y).abs() < EPSILON,
        "y: expected {}, got {}",
        expected.y,
        actual.y,
    );

    assert!(
        (actual.z - expected.z).abs() < EPSILON,
        "z: expected {}, got {}",
        expected.z,
        actual.z,
    );
}

fn empty_mesh() -> Mesh {
    Mesh::new(
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn attachment_points_coincide() {
    let parent =
        empty_mesh();

    let child =
        empty_mesh();

    let mut parent_transform =
        Transform::new();

    parent_transform.set_position(
        Vec3::new(
            10.0,
            5.0,
            -3.0,
        )
    );

    let parent_attach =
        Vec3::new(
            1.0,
            0.0,
            0.0,
        );

    let child_attach =
        Vec3::new(
            0.0,
            2.0,
            0.0,
        );

    let result =
        from_attachment(
            &parent,
            &parent_transform,
            parent_attach,

            &child,
            child_attach,

            Vec3::new(
                0.0,
                0.0,
                -1.0,
            ),

            Angle::ZERO,
            Vec3::new(1.0, 1.0, 1.0),
        );

    // Parent has no rotation or scale, so its
    // attachment point should simply be translated.
    let expected_parent_world =
        Vec3::new(
            11.0,
            5.0,
            -3.0,
        );

    // Determine where the child's attachment point
    // actually ends up using the returned Transform.
    let child_offset =
        child_attach - child.pivot();

    let rotated_offset =
        result
            .rotation()
            .rotate_vector(
                child_offset
            );

    let actual_child_world =
        result.position()
            + rotated_offset;

    assert_vec3_close(
        actual_child_world,
        expected_parent_world,
    );
}

#[test]
fn child_direction_is_aligned() {
    let parent =
        empty_mesh();

    let child =
        empty_mesh();

    let desired_direction =
        Vec3::new(
            1.0,
            1.0,
            -1.0,
        )
        .normalized();

    let result =
        from_attachment(
            &parent,
            &Transform::new(),
            Vec3::default(),

            &child,
            Vec3::default(),

            desired_direction,

            Angle::ZERO,
            Vec3::new(1.0, 1.0, 1.0),
        );

    let actual_direction =
        result
            .rotation()
            .rotate_vector(
                child.direction()
            )
            .normalized();

    assert_vec3_close(
        actual_direction,
        desired_direction,
    );
}

#[test]
fn zero_rotation_follows_parent_reference() {
    let parent =
        empty_mesh();

    let child =
        empty_mesh();

    let mut parent_transform =
        Transform::new();

    parent_transform.set_euler(
        Angle::ZERO,
        Angle::ZERO,
        Angle::degrees(45.0),
    );

    let desired_direction =
        Vec3::new(
            0.0,
            0.0,
            -1.0,
        );

    let result =
        from_attachment(
            &parent,
            &parent_transform,
            Vec3::default(),

            &child,
            Vec3::default(),

            desired_direction,

            Angle::ZERO,
            Vec3::new(1.0, 1.0, 1.0),
        );

    let parent_reference =
        parent_transform
            .rotation()
            .rotate_vector(
                parent.reference()
            )
            .normalized();

    let child_reference =
        result
            .rotation()
            .rotate_vector(
                child.reference()
            )
            .normalized();

    assert_vec3_close(
        child_reference,
        parent_reference,
    );
}

#[test]
fn rotation_twists_around_direction() {
    let parent =
        empty_mesh();

    let child =
        empty_mesh();

    let direction =
        Vec3::new(
            0.0,
            0.0,
            -1.0,
        );

    let result =
        from_attachment(
            &parent,
            &Transform::new(),
            Vec3::default(),

            &child,
            Vec3::default(),

            direction,

            Angle::degrees(90.0),
            Vec3::new(1.0, 1.0, 1.0),
        );

    let actual_direction =
        result
            .rotation()
            .rotate_vector(
                child.direction()
            )
            .normalized();

    // Twisting must not change the primary direction.
    assert_vec3_close(
        actual_direction,
        direction,
    );

    let actual_reference =
        result
            .rotation()
            .rotate_vector(
                child.reference()
            )
            .normalized();

    // Default reference is +Y. Rotating +90 degrees
    // around -Z should point it toward +X.
    assert_vec3_close(
        actual_reference,
        Vec3::new(
            1.0,
            0.0,
            0.0,
        ),
    );
}

#[test]
fn parallel_parent_reference_uses_side_fallback() {
    let parent =
        empty_mesh();

    let child =
        empty_mesh();

    // Parent reference defaults to +Y.
    // Deliberately request +Y as the child's direction,
    // making the parent reference unusable as a projected
    // zero-twist reference.
    let desired_direction =
        Vec3::new(
            0.0,
            1.0,
            0.0,
        );

    let result =
        from_attachment(
            &parent,
            &Transform::new(),
            Vec3::default(),

            &child,
            Vec3::default(),

            desired_direction,

            Angle::ZERO,
            Vec3::new(1.0, 1.0, 1.0),
        );

    let actual_direction =
        result
            .rotation()
            .rotate_vector(
                child.direction()
            )
            .normalized();

    assert_vec3_close(
        actual_direction,
        desired_direction,
    );

    // Most importantly, the resulting orientation
    // must remain a valid normalized frame.
    let reference =
        result
            .rotation()
            .rotate_vector(
                child.reference()
            )
            .normalized();

    assert!(
        reference
            .dot(&actual_direction)
            .abs()
            < EPSILON
    );
}
