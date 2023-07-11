use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn with_values(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(self) -> Point3 {
        return self.origin;
    }

    pub fn direction(self) -> Vec3 {
        return self.direction;
    }

    pub fn at(self, t: f32) -> Point3 {
        return self.origin + self.direction * t;
    }
}
