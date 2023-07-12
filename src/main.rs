mod color;
mod ray;
mod vec3;

use color::write_color;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::with_values(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let diff = r.at(t) - Vec3::with_values(0.0, 0.0, -1.0);
        let N = vec3::unit_vector(&diff);
        return 0.5 * Color::with_values(N.x(), N.y(), N.z());
    }

    let unit_direction = vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::with_values(1.0, 1.0, 1.0) + t * Color::with_values(0.5, 0.7, 1.0);
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(&oc, &r.direction());
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn main() {
    //Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width / aspect_ratio as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Point3::with_values(0.0, 0.0, 0.0);
    let horizontal = Vec3::with_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::with_values(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::with_values(0.0, 0.0, focal_length);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i / image_width - 1;
            let v = j / image_height - 1;
            let r = Ray::with_values(
                origin,
                lower_left_corner + u as f32 * horizontal + v as f32 * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }
}
