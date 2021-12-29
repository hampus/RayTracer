use crate::common::Direction;
use crate::common::Float;
use crate::common::Point;
use crate::common::Ray;
use nalgebra::Isometry3;

use nalgebra::point;
use nalgebra::vector;
use nalgebra::Matrix4;
use nalgebra::Point2;

use nalgebra::Transform3;
use nalgebra::Unit;

pub struct Camera {
    pub origin: Point,         // Origin of the lens
    pub direction: Direction,  // Direction that the lens is looking
    pub focal_length: Float,   // Assuming 35mm sensor (36x24mm)
    pub focus_distance: Float, // Distance from the lens to the focal plane
    pub f_number: Float,       // f-number: f/f_number
    transform: Transform3<Float>,
    lens_transformation: Transform3<Float>,
}

const SENSOR_DIAGONAL_MM: Float = 43.3;
const SENSOR_HEIGHT_MM: Float = 24.0;

impl Camera {
    pub fn new(
        origin: Point,
        look_at: Point,
        field_of_view_height_degrees: Float,
        f_number: Float,
        aspect_ratio: Float,
    ) -> Camera {
        let fov = field_of_view_height_degrees / 180.0 * std::f64::consts::PI;
        let focal_length = (SENSOR_DIAGONAL_MM / 1000.0 / 2.0) / (fov / 2.0).tan();
        let focus_vector = look_at - origin;
        let focus_distance = focus_vector.norm();

        let scale_y = SENSOR_HEIGHT_MM / 1000.0 / 2.0 * (focus_distance / focal_length);
        let scale_x = scale_y * aspect_ratio;
        let scale = Matrix4::new_nonuniform_scaling(&vector![scale_x, scale_y, 1.0]);
        let translate = Matrix4::new_translation(&vector![0.0, 0.0, -focus_distance]);
        let isometry = Isometry3::face_towards(
            &origin,
            &(origin + (origin - look_at)),
            &vector![0.0, 1.0, 0.0],
        );
        let transform = isometry.to_homogeneous() * translate * scale;

        let aperture_radius = if f_number == 0.0 {
            0.0
        } else {
            focal_length / f_number / 2.0
        };
        let lens_transformation = isometry.to_homogeneous() * Matrix4::new_scaling(aperture_radius);

        Camera {
            origin,
            direction: Unit::new_normalize(focus_vector),
            focal_length,
            focus_distance: focus_vector.norm(),
            f_number,
            transform: Transform3::from_matrix_unchecked(transform),
            lens_transformation: Transform3::from_matrix_unchecked(lens_transformation),
        }
    }

    pub fn generate_ray(
        &self,
        screen_position: Point2<Float>,
        lens_position: Point2<Float>,
    ) -> Ray {
        let screen_3d_point: Point = point![screen_position.x, screen_position.y, 0.0];
        let origin = self.lens_transformation * point![lens_position.x, lens_position.y, 0.0];
        Ray {
            origin,
            direction: Unit::new_normalize(self.transform * screen_3d_point - origin),
        }
    }
}
