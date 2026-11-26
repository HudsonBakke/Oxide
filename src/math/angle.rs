//! Angle values with explicit degree and radian conversions.

/// An angle stored internally in radians.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    /// An angle of zero radians.
    pub const ZERO: Self = Self { radians: 0.0 };

    /// Creates an angle from radians.
    pub const fn radians(radians: f32) -> Self {
        Self { radians }
    }

    /// Creates an angle from degrees.
    pub fn degrees(degrees: f32) -> Self {
        Self {
            radians: degrees.to_radians(),
        }
    }

    /// Returns this angle in radians.
    pub const fn as_radians(self) -> f32 {
        self.radians
    }

    /// Returns this angle in degrees.
    pub fn as_degrees(self) -> f32 {
        self.radians.to_degrees()
    }
}
