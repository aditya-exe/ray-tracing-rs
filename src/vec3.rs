use rand::prelude::*;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn with_values(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            e: [rng.gen(), rng.gen(), rng.gen()],
        }
    }

    pub fn random_init(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            e: [
                rng.gen_range(min..max),
                rng.gen_range(min..max),
                rng.gen_range(min..max),
            ],
        }
    }

    pub fn x(&self) -> f32 {
        return self.e[0];
    }

    pub fn y(&self) -> f32 {
        return self.e[1];
    }

    pub fn z(&self) -> f32 {
        return self.e[2];
    }

    pub fn index(&self, i: usize) -> f32 {
        return self.e[i];
    }

    pub fn index_mut(&mut self, i: usize) -> &mut f32 {
        return &mut self.e[i];
    }

    pub fn length(&self) -> f32 {
        return Self::length_squared(&self).sqrt();
    }

    pub fn length_squared(&self) -> f32 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();

        Vec3 { e: [x, y, z] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();

        *self = Self { e: [x, y, z] }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();

        Vec3 { e: [x, y, z] }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Vec3 {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        let z = self.z() * rhs.z();

        Vec3 { e: [x, y, z] }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        let z = self.z() * rhs.z();

        *self = Self { e: [x, y, z] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            e: [t * self.e[0], t * self.e[1], t * self.e[2]],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Vec3 {
        let x = self.x() / rhs.x();
        let y = self.y() / rhs.y();
        let z = self.z() / rhs.z();

        Vec3 { e: [x, y, z] }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        let x = self.x() / rhs.x();
        let y = self.y() / rhs.y();
        let z = self.z() / rhs.z();

        *self = Self { e: [x, y, z] }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        let inv_t = 1.0 / t;
        Vec3 {
            e: [self.e[0] * inv_t, self.e[1] * inv_t, self.e[2] * inv_t],
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    return Vec3::with_values(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    );
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    let x = Vec3 { e: v.e };
    let len = v.length();

    return x / len;
}

pub type Point3 = Vec3;
pub type Color = Vec3;
