use nalgebra;
use std::sync::Arc;

pub type Float = f64;
pub type Point = nalgebra::Point3<Float>;
pub type Vector = nalgebra::Vector3<Float>;
pub type Direction = nalgebra::Unit<Vector>;

pub const INFINITY: Float = Float::INFINITY;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}

impl Ray {
    pub fn at(&self, distance: Float) -> Point {
        self.origin + distance * self.direction.into_inner()
    }
}

#[derive(Debug)]
pub struct RayIntersection {
    pub position: Point,
    pub normal: Direction,
    pub distance: Float,
    pub material: Arc<Box<dyn Material>>,
}

#[derive(Debug)]
pub struct ScatteredRay {
    pub attenuation: Vector,
    pub ray: Ray,
}

pub trait Material: std::fmt::Debug + Sync + Send {
    fn scatter_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<ScatteredRay>;
}

pub trait RayTracable: Sync + Send {
    fn trace_ray(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<RayIntersection>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra;

    #[test]
    fn ray_at_gives_expected_values() {
        let ray = Ray {
            origin: nalgebra::point![1.0, 2.0, 3.0],
            direction: nalgebra::Unit::new_normalize(nalgebra::vector![1.0, 0.0, 0.0]),
        };
        assert_eq!(ray.at(0.0), ray.origin);
        assert_eq!(ray.at(1.0), nalgebra::point![2.0, 2.0, 3.0]);
    }
}
