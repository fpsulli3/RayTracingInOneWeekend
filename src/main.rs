use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Write;
use crate::math::Vec3;
use rand::prelude::*;

mod math;

fn main() {
    let path = Path::new("output/rendered_image.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let nx = 1280;
    let ny = 720;
    let ns = 128;

    file.write("P3\n".as_bytes());
    file.write_fmt(format_args!("{} {}\n", nx, ny));
    file.write("255\n".as_bytes());

    let mut a = math::Vec3{ x: 10.0, y: 10.0, z: 10.0 };
    a.normalize();
    let x = a.x;
    print!("x: {}", x);

    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut rgb = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
            for s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / (nx as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (ny as f32);

                let ray = get_cam_ray(u, v);
                rgb += color(&ray);
            }

            rgb /= (ns as f32);

            let ir = (255.99 * rgb.x) as i32;
            let ig = (255.99 * rgb.y) as i32;
            let ib = (255.99 * rgb.z) as i32;
            file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
        }
    }
}

fn get_cam_ray(u: f32, v: f32) -> math::Ray {
    let lower_left_corner = math::Vec3 { x: -1.7778, y: -1.0, z: -1.0 };
    let horizontal = math::Vec3 { x: 3.556, y: 0.0, z: 0.0 };
    let vertical = math::Vec3 { x: 0.0, y: 2.0, z: 0.0 };
    let origin = math::Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    return math::Ray {
        origin,
        direction: lower_left_corner + u*horizontal + v*vertical,
    };
}

fn color(r: &math::Ray) -> math::Vec3 {
    let sphere_center = Vec3 {x: 0.0, y: 0.0, z: -1.0};
    let t = hit_sphere(&sphere_center, 0.5, r);
    if t > 0.0 {
        let n = (&r.point_at_parameter(t) - sphere_center).normalized();
        return 0.5*Vec3 {x: n.x + 1.0, y: n.y + 1.0, z: n.z + 1.0}
    }
    let unit_direction = r.direction.normalized();
    let t = 0.5*(unit_direction.y + 1.0);

    return (1.0 - t)*math::Vec3 {x: 1.0, y: 1.0, z: 1.0} + t*math::Vec3 {x: 0.5, y: 0.7, z: 1.0};
}

fn hit_sphere(center: &math::Vec3, radius: f32, ray: &math::Ray) -> f32 {
    let oc = &ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0*a);
    }
}

fn random_in_unit_sphere() -> math::Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    // crappy do...while hack
    while {
        p = Vec3 { x: rng.gen(), y: rng.gen(), z: rng.gen() } * 2.0 - 1.0;
        p.length_squared() >= 1.0
    } {}

    return p
}



