use crate::math::{
    Vec3,
    Mat4,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn empty() -> Self {
        Self {
            min: Vec3::new(
                f32::INFINITY,
                f32::INFINITY,
                f32::INFINITY,
            ),

            max: Vec3::new(
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
            ),
        }
    }

    pub fn from_triangle(
        a: Vec3,
        b: Vec3,
        c: Vec3,
    ) -> Self {
        let mut result = Self::empty();

        result.expand_point(a);
        result.expand_point(b);
        result.expand_point(c);

        result
    }

    pub fn expand_point(
        &mut self,
        point: Vec3,
    ) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);

        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }

    pub fn expand_aabb(
        &mut self,
        other: &Self,
    ) {
        self.expand_point(other.min);
        self.expand_point(other.max);
    }

    pub fn centroid(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn surface_area(&self) -> f32 {
        let extent = self.max - self.min;

        2.0 * (
            extent.x * extent.y
            + extent.y * extent.z
            + extent.z * extent.x
        )
    }

    pub fn intersects(
        &self,
        other: &Self,
    ) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn transformed(
        &self,
        transform: &Mat4,
    ) -> Self {
        let center =
            (self.min + self.max) * 0.5;

        let extents =
            (self.max - self.min) * 0.5;

        let transformed_center =
            transform.transform_point(center);

        let transformed_extents =
            Vec3::new(
                transform.get(0, 0).abs() * extents.x
                    + transform.get(0, 1).abs() * extents.y
                    + transform.get(0, 2).abs() * extents.z,

                transform.get(1, 0).abs() * extents.x
                    + transform.get(1, 1).abs() * extents.y
                    + transform.get(1, 2).abs() * extents.z,

                transform.get(2, 0).abs() * extents.x
                    + transform.get(2, 1).abs() * extents.y
                    + transform.get(2, 2).abs() * extents.z,
            );

        Self {
            min:
                transformed_center
                - transformed_extents,

            max:
                transformed_center
                + transformed_extents,
        }
    }

    pub fn intersects_ray(
        &self,
        origin: Vec3,
        direction: Vec3,
    ) -> bool {
        let mut t_min = 0.0f32;
        let mut t_max = f32::INFINITY;

        for axis in 0..3 {
            let origin_component = match axis {
                0 => origin.x,
                1 => origin.y,
                2 => origin.z,
                _ => unreachable!(),
            };

            let direction_component = match axis {
                0 => direction.x,
                1 => direction.y,
                2 => direction.z,
                _ => unreachable!(),
            };

            let min_component = match axis {
                0 => self.min.x,
                1 => self.min.y,
                2 => self.min.z,
                _ => unreachable!(),
            };

            let max_component = match axis {
                0 => self.max.x,
                1 => self.max.y,
                2 => self.max.z,
                _ => unreachable!(),
            };

            if direction_component.abs() <= f32::EPSILON {
                if origin_component < min_component
                    || origin_component > max_component
                {
                    return false;
                }

                continue;
            }

            let inverse_direction =
                1.0 / direction_component;

            let mut t1 =
                (min_component - origin_component)
                    * inverse_direction;

            let mut t2 =
                (max_component - origin_component)
                    * inverse_direction;

            if t1 > t2 {
                std::mem::swap(
                    &mut t1,
                    &mut t2,
                );
            }

            t_min = t_min.max(t1);
            t_max = t_max.min(t2);

            if t_min > t_max {
                return false;
            }
        }

        true
    }
}
