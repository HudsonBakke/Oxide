use crate::math::{
    Mat4,
    Vec3,
};

const EPSILON: f32 = 1.0e-6;

#[derive(Debug, Clone, Copy)]
pub(super) struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(
        a: Vec3,
        b: Vec3,
        c: Vec3,
    ) -> Self {
        Self { a, b, c }
    }

    pub fn normal(&self) -> Vec3 {
        (self.b - self.a)
            .cross(&(self.c - self.a))
    }

    pub fn is_degenerate(&self) -> bool {
        self.normal().length() <= EPSILON
    }
}

fn segment_intersects_triangle(
    start: Vec3,
    end: Vec3,
    triangle: &Triangle,
) -> bool {
    let direction = end - start;

    let edge1 =
        triangle.b - triangle.a;

    let edge2 =
        triangle.c - triangle.a;

    let h =
        direction.cross(&edge2);

    let determinant =
        edge1.dot(&h);

    if determinant.abs() <= EPSILON {
        return false;
    }

    let inverse_determinant =
        1.0 / determinant;

    let s =
        start - triangle.a;

    let u =
        inverse_determinant
        * s.dot(&h);

    if u < -EPSILON
        || u > 1.0 + EPSILON
    {
        return false;
    }

    let q =
        s.cross(&edge1);

    let v =
        inverse_determinant
        * direction.dot(&q);

    if v < -EPSILON
        || u + v > 1.0 + EPSILON
    {
        return false;
    }

    let t =
        inverse_determinant
        * edge2.dot(&q);

    t >= -EPSILON
        && t <= 1.0 + EPSILON
}

fn signed_plane_distance(
    point: Vec3,
    plane_point: Vec3,
    plane_normal: Vec3,
) -> f32 {
    (point - plane_point)
        .dot(&plane_normal)
}

fn are_coplanar(
    first: &Triangle,
    second: &Triangle,
) -> bool {
    let normal =
        first.normal();

    let length =
        normal.length();

    if length <= EPSILON {
        return false;
    }

    let normal =
        normal / length;

    signed_plane_distance(
        second.a,
        first.a,
        normal,
    ).abs() <= EPSILON
        &&
    signed_plane_distance(
        second.b,
        first.a,
        normal,
    ).abs() <= EPSILON
        &&
    signed_plane_distance(
        second.c,
        first.a,
        normal,
    ).abs() <= EPSILON
}

#[derive(Debug, Clone, Copy)]
struct Vec2Projection {
    x: f32,
    y: f32,
}

fn project_point(
    point: Vec3,
    dropped_axis: usize,
) -> Vec2Projection {
    match dropped_axis {
        0 => Vec2Projection {
            x: point.y,
            y: point.z,
        },

        1 => Vec2Projection {
            x: point.x,
            y: point.z,
        },

        2 => Vec2Projection {
            x: point.x,
            y: point.y,
        },

        _ => unreachable!(),
    }
}

fn dominant_axis(
    normal: Vec3,
) -> usize {
    let x = normal.x.abs();
    let y = normal.y.abs();
    let z = normal.z.abs();

    if x >= y && x >= z {
        0
    } else if y >= z {
        1
    } else {
        2
    }
}

fn orient_2d(
    a: Vec2Projection,
    b: Vec2Projection,
    c: Vec2Projection,
) -> f32 {
    (b.x - a.x) * (c.y - a.y)
        - (b.y - a.y) * (c.x - a.x)
}

fn point_on_segment_2d(
    point: Vec2Projection,
    a: Vec2Projection,
    b: Vec2Projection,
) -> bool {
    orient_2d(a, b, point).abs()
        <= EPSILON
        &&
    point.x
        >= a.x.min(b.x) - EPSILON
        &&
    point.x
        <= a.x.max(b.x) + EPSILON
        &&
    point.y
        >= a.y.min(b.y) - EPSILON
        &&
    point.y
        <= a.y.max(b.y) + EPSILON
}

fn segments_intersect_2d(
    a1: Vec2Projection,
    a2: Vec2Projection,
    b1: Vec2Projection,
    b2: Vec2Projection,
) -> bool {
    let o1 =
        orient_2d(a1, a2, b1);

    let o2 =
        orient_2d(a1, a2, b2);

    let o3 =
        orient_2d(b1, b2, a1);

    let o4 =
        orient_2d(b1, b2, a2);

    if (
        (o1 > EPSILON && o2 < -EPSILON)
        || (o1 < -EPSILON && o2 > EPSILON)
    ) && (
        (o3 > EPSILON && o4 < -EPSILON)
        || (o3 < -EPSILON && o4 > EPSILON)
    ) {
        return true;
    }

    if o1.abs() <= EPSILON
        && point_on_segment_2d(
            b1, a1, a2
        )
    {
        return true;
    }

    if o2.abs() <= EPSILON
        && point_on_segment_2d(
            b2, a1, a2
        )
    {
        return true;
    }

    if o3.abs() <= EPSILON
        && point_on_segment_2d(
            a1, b1, b2
        )
    {
        return true;
    }

    if o4.abs() <= EPSILON
        && point_on_segment_2d(
            a2, b1, b2
        )
    {
        return true;
    }

    false
}

fn point_in_triangle_2d(
    point: Vec2Projection,
    a: Vec2Projection,
    b: Vec2Projection,
    c: Vec2Projection,
) -> bool {
    let d1 =
        orient_2d(a, b, point);

    let d2 =
        orient_2d(b, c, point);

    let d3 =
        orient_2d(c, a, point);

    let has_negative =
        d1 < -EPSILON
        || d2 < -EPSILON
        || d3 < -EPSILON;

    let has_positive =
        d1 > EPSILON
        || d2 > EPSILON
        || d3 > EPSILON;

    !(has_negative && has_positive)
}

fn coplanar_intersects(
    first: &Triangle,
    second: &Triangle,
) -> bool {
    let axis =
        dominant_axis(first.normal());

    let a = [
        project_point(first.a, axis),
        project_point(first.b, axis),
        project_point(first.c, axis),
    ];

    let b = [
        project_point(second.a, axis),
        project_point(second.b, axis),
        project_point(second.c, axis),
    ];

    let edges = [
        (0, 1),
        (1, 2),
        (2, 0),
    ];

    for &(a0, a1) in &edges {
        for &(b0, b1) in &edges {
            if segments_intersect_2d(
                a[a0],
                a[a1],
                b[b0],
                b[b1],
            ) {
                return true;
            }
        }
    }

    point_in_triangle_2d(
        a[0],
        b[0],
        b[1],
        b[2],
    ) || point_in_triangle_2d(
        b[0],
        a[0],
        a[1],
        a[2],
    )
}

impl Triangle {
    pub fn intersects(
        &self,
        other: &Self,
    ) -> bool {
        if self.is_degenerate()
            || other.is_degenerate()
        {
            return false;
        }

        if are_coplanar(self, other) {
            return coplanar_intersects(
                self,
                other,
            );
        }

        let self_edges = [
            (self.a, self.b),
            (self.b, self.c),
            (self.c, self.a),
        ];

        for &(start, end) in &self_edges {
            if segment_intersects_triangle(
                start,
                end,
                other,
            ) {
                return true;
            }
        }

        let other_edges = [
            (other.a, other.b),
            (other.b, other.c),
            (other.c, other.a),
        ];

        for &(start, end) in &other_edges {
            if segment_intersects_triangle(
                start,
                end,
                self,
            ) {
                return true;
            }
        }

        false
    }

    pub fn transformed(
        &self,
        transform: &Mat4,
    ) -> Self {
        Self {
            a: transform.transform_point(self.a),
            b: transform.transform_point(self.b),
            c: transform.transform_point(self.c),
        }
    }

    pub fn ray_intersection_distance(
        &self,
        origin: Vec3,
        direction: Vec3,
    ) -> Option<f32> {
        if self.is_degenerate() {
            return None;
        }

        let edge1 =
            self.b - self.a;

        let edge2 =
            self.c - self.a;

        let h =
            direction.cross(&edge2);

        let determinant =
            edge1.dot(&h);

        if determinant.abs() <= EPSILON {
            return None;
        }

        let inverse_determinant =
            1.0 / determinant;

        let s =
            origin - self.a;

        let u =
            inverse_determinant
                * s.dot(&h);

        if u < -EPSILON
            || u > 1.0 + EPSILON
        {
            return None;
        }

        let q =
            s.cross(&edge1);

        let v =
            inverse_determinant
                * direction.dot(&q);

        if v < -EPSILON
            || u + v > 1.0 + EPSILON
        {
            return None;
        }

        let distance =
            inverse_determinant
                * edge2.dot(&q);

        if distance > EPSILON {
            Some(distance)
        } else {
            None
        }
    }
}
