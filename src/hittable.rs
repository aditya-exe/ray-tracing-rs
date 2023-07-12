use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

// trait FrontFace {
//     fn set_front_face(&mut self, r: &Ray, outward_normal: &Vec3);
// }

impl HitRecord {
    pub fn set_front_face(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
