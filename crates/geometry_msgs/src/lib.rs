use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

impl Twist {
    pub const ZERO: Self = Self {
        linear: Vector3::ZERO,
        angular: Vector3::ZERO,
    };

    pub fn new(linear: Vector3, angular: Vector3) -> Self {
        Self { linear, angular }
    }
}

impl Default for Twist {
    fn default() -> Self {
        Self::ZERO
    }
}
