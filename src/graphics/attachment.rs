use crate::{
    graphics::Mesh,
    math::{
        Angle,
        Quaternion,
        Transform,
        Vec3,
    },
};

const ATTACHMENT_EPSILON: f32 = 1.0e-6;

/// Creates a child transform that attaches one model-space point
/// to a transformed parent mesh.
///
/// The child's canonical direction is aligned with `direction`.
/// At zero additional rotation, the child's canonical reference
/// direction follows the parent's orientation as closely as possible.
///
/// `rotation` then applies an additional twist around `direction`.
pub fn from_attachment(
    parent: &Mesh,
    parent_transform: &Transform,
    parent_attach_point: Vec3,

    child: &Mesh,
    child_attach_point: Vec3,

    direction: Vec3,
    rotation: Angle,
    scale: Vec3,
) -> crate::OxideResult<Transform> {
    if direction.length()
        <= ATTACHMENT_EPSILON
    {
        return Err(
            crate::OxideError::InvalidArgument {
                argument:
                    "direction",

                message:
                    "attachment direction cannot be zero",
            }
        );
    }

    let direction = direction.normalized();

    // ---------------------------------------------------------
    // 1. Determine the parent's orientation in world space.
    // ---------------------------------------------------------

    let parent_rotation =
        parent_transform.rotation();

    let parent_reference =
        parent_rotation
            .rotate_vector(
                parent.reference()
            )
            .normalized();

    let parent_side =
        parent_rotation
            .rotate_vector(
                parent.side()
            )
            .normalized();

    // ---------------------------------------------------------
    // 2. Determine the desired zero-twist reference direction.
    //
    // Project the parent's reference onto the plane
    // perpendicular to the desired child direction.
    // ---------------------------------------------------------

    let mut target_reference =
        parent_reference
            - direction
                * parent_reference.dot(
                    &direction
                );

    // The parent's reference may be parallel to the desired
    // child direction. In that case, its projection has no
    // useful direction, so use the parent's side axis instead.
    if target_reference.length()
        <= ATTACHMENT_EPSILON
    {
        target_reference =
            parent_side
                - direction
                    * parent_side.dot(
                        &direction
                    );
    }

    if target_reference.length()
        <= ATTACHMENT_EPSILON
    {
        return Err(
            crate::OxideError::InvalidOperation {
                message:
                    "could not determine attachment reference orientation",
            }
        );
    }

    target_reference.normalize();

    // ---------------------------------------------------------
    // 3. Align the child's canonical direction with the
    // requested world-space direction.
    // ---------------------------------------------------------

    let alignment =
        Quaternion::from_to(
            child.direction(),
            direction,
        );

    let aligned_reference =
        alignment
            .rotate_vector(
                child.reference()
            )
            .normalized();

    // ---------------------------------------------------------
    // 4. Determine the signed twist required to align the
    // child's reference with the parent's projected reference.
    //
    // atan2 gives us both magnitude and direction of rotation.
    // ---------------------------------------------------------

    let cross =
        aligned_reference.cross(
            &target_reference
        );

    let sin =
        direction.dot(&cross);

    let cos =
        aligned_reference.dot(
            &target_reference
        );

    let baseline_angle =
        sin.atan2(cos);

    let baseline_twist =
        Quaternion::from_axis_angle(
            direction,
            Angle::radians(
                baseline_angle
            ),
        );

    // ---------------------------------------------------------
    // 5. Apply the user's additional twist around the final
    // child direction.
    // ---------------------------------------------------------

    let user_twist =
        Quaternion::from_axis_angle(
            direction,
            rotation,
        );

    let orientation =
        (
            user_twist
            * baseline_twist
            * alignment
        )
        .normalized();

    // ---------------------------------------------------------
    // 6. Determine the parent's attachment point in world space.
    // ---------------------------------------------------------

    let parent_matrix =
        parent_transform.matrix_around(
            parent.pivot()
        );

    let parent_world_attach =
        parent_matrix.transform_point(
            parent_attach_point
        );

    // ---------------------------------------------------------
    // 7. Solve the child's position.
    //
    // Transform::position represents the world-space position
    // of the mesh pivot. Therefore calculate the vector from
    // the child's pivot to its attachment point, apply the
    // child's scale and rotation, and place the pivot so that
    // the attachment point lands exactly on the parent's.
    // ---------------------------------------------------------

    let child_scale =
        scale;

    let child_local_offset =
        child_attach_point
            - child.pivot();

    let scaled_child_offset =
        Vec3::new(
            child_local_offset.x
                * child_scale.x,

            child_local_offset.y
                * child_scale.y,

            child_local_offset.z
                * child_scale.z,
        );

    let child_world_offset =
        orientation.rotate_vector(
            scaled_child_offset
        );

    let child_position =
        parent_world_attach
            - child_world_offset;

    // ---------------------------------------------------------
    // 8. Construct the resulting transform.
    // ---------------------------------------------------------

    let mut result =
        Transform::new();

    result.set_position(
        child_position
    );

    result.set_rotation(
        orientation
    );

    result.set_scale(
        child_scale
    );

    Ok(result)
}