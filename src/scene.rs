use crate::common::Float;
use crate::common::Point;
use crate::common::Ray;
use crate::common::RayIntersection;
use crate::common::RayTracable;
use nalgebra::vector;
use nalgebra::Unit;

pub struct SceneList {
    pub objects: Vec<Box<dyn RayTracable>>,
}

impl RayTracable for SceneList {
    fn trace_ray(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<RayIntersection> {
        let mut closest_dist = max_dist;
        let mut closest_intersection: Option<RayIntersection> = None;

        for object in &self.objects {
            if let Some(intersection) = object.trace_ray(ray, min_dist, closest_dist) {
                closest_dist = intersection.distance;
                closest_intersection = Some(intersection);
            }
        }

        closest_intersection
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: Float,
}

impl RayTracable for Sphere {
    fn trace_ray(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<RayIntersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&oc);
        let delta = a.powi(2) - (oc.norm_squared() - self.radius.powi(2));

        if delta < 0.0 {
            return None;
        }

        let sqrt_delta = delta.sqrt();
        let first_distance = -a - sqrt_delta;
        let second_distance = -a + sqrt_delta;

        let distance = if first_distance >= min_dist {
            first_distance
        } else {
            second_distance
        };

        if distance < min_dist || distance > max_dist {
            return None;
        }

        let position = ray.at(distance);
        Some(RayIntersection {
            distance,
            position,
            normal: Unit::new_unchecked((position - self.center) / self.radius),
        })
    }
}

pub struct Floor {
    pub y: Float,
}

impl RayTracable for Floor {
    fn trace_ray(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<RayIntersection> {
        if ray.direction.y.abs() < Float::EPSILON {
            return None;
        }
        let distance = (self.y - ray.origin.y) / ray.direction.y;
        if distance < min_dist || distance > max_dist {
            return None;
        }
        Some(RayIntersection {
            distance,
            position: ray.at(distance),
            normal: Unit::new_unchecked(vector![0.0, 1.0, 0.0]),
        })
    }
}
