use crate::common::Direction;
use crate::common::Float;
use crate::common::Material;
use crate::common::Ray;
use crate::common::RayIntersection;
use crate::common::ScatteredRay;
use crate::common::Vector;
use crate::srgb::srgb_to_rgb;
use nalgebra::vector;
use nalgebra::Unit;
use rand::prelude::*;
use rand::thread_rng;

#[derive(Debug)]
pub struct Lambertian {
    pub color: Vector,
}

impl Material for Lambertian {
    fn scatter_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<ScatteredRay> {
        Some(ScatteredRay {
            ray: generate_lambertian_ray(intersection),
            attenuation: self.color,
        })
    }
}

#[derive(Debug)]
pub struct Metal {
    pub color: Vector,
}

impl Material for Metal {
    fn scatter_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<ScatteredRay> {
        let reflection = Unit::new_normalize(
            2.0 * (-ray.direction).dot(&intersection.normal) * intersection.normal.into_inner()
                + ray.direction.into_inner(),
        );
        Some(ScatteredRay {
            ray: generate_reflection_ray(ray, intersection),
            attenuation: self.color,
        })
    }
}

#[derive(Debug)]
pub struct MixedMaterial {
    pub color: Vector,
    pub shininess: Float,
}

impl Material for MixedMaterial {
    fn scatter_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<ScatteredRay> {
        let mut rng = thread_rng();
        let scattered_ray = if rng.gen::<Float>() < self.shininess {
            generate_reflection_ray(ray, intersection)
        } else {
            generate_lambertian_ray(intersection)
        };
        Some(ScatteredRay {
            ray: scattered_ray,
            attenuation: self.color,
        })
    }
}

#[derive(Debug)]
pub struct FloorMaterial {
    pub color: Vector,
}
impl Material for FloorMaterial {
    fn scatter_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<ScatteredRay> {
        let position = intersection.position;
        Some(ScatteredRay {
            ray: generate_lambertian_ray(intersection),
            attenuation: if ((position.x.round() as i64) + (position.z.round() as i64)) % 2 == 0 {
                srgb_to_rgb(self.color)
            } else {
                srgb_to_rgb(vector![0.2, 0.2, 0.2])
            },
        })
    }
}

fn random_direction_on_hemisphere_cosine_weighted(normal: &Direction) -> Direction {
    let mut rng = thread_rng();
    loop {
        let v = vector![rng.gen::<Float>(), rng.gen::<Float>(), rng.gen::<Float>()];
        let d = Unit::new_normalize((v - vector![0.5, 0.5, 0.5]) * 2.0);
        if d.norm_squared() <= 1.0 {
            let cos_of_normal_angle = d.dot(normal);
            // Accept direction with probability relative to cos(normal_angle)
            if rng.gen::<Float>() <= cos_of_normal_angle.abs() {
                if cos_of_normal_angle < 0.0 {
                    return -d;
                } else {
                    return d;
                }
            }
        }
    }
}

fn generate_lambertian_ray(intersection: &RayIntersection) -> Ray {
    Ray {
        origin: intersection.position,
        direction: random_direction_on_hemisphere_cosine_weighted(&intersection.normal),
    }
}

fn generate_reflection_ray(ray: &Ray, intersection: &RayIntersection) -> Ray {
    let reflection = Unit::new_normalize(
        2.0 * (-ray.direction).dot(&intersection.normal) * intersection.normal.into_inner()
            + ray.direction.into_inner(),
    );
    Ray {
        origin: intersection.position,
        direction: reflection,
    }
}
