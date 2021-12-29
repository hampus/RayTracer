use image::RgbImage;
use nalgebra::point;
use nalgebra::vector;
use raytracer::camera::Camera;
use raytracer::render::render;
use raytracer::render::RenderConfig;
use raytracer::scene::Floor;
use raytracer::scene::SceneList;
use raytracer::scene::Sphere;
use raytracer::srgb::srgb_to_rgb;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 1920;
    let config = RenderConfig {
        width,
        height: ((width as f64) / aspect_ratio).round() as u32,
        aspect_ratio,
        samples_per_pixel: 2000,
        max_depth: 50,
        tile_size: 16,
    };

    let scene = SceneList {
        objects: vec![
            Box::new(Sphere {
                center: point![0.0, 1.0, -5.0],
                radius: 1.0,
                color: srgb_to_rgb(vector![1.0, 0.5, 0.5]),
            }),
            Box::new(Sphere {
                center: point![-1.5, 0.5, -5.0],
                radius: 0.5,
                color: srgb_to_rgb(vector![0.5, 0.6, 1.0]),
            }),
            Box::new(Sphere {
                center: point![1.5, 0.5, -3.5],
                radius: 0.5,
                color: srgb_to_rgb(vector![0.5, 0.6, 1.0]),
            }),
            Box::new(Sphere {
                center: point![4.5, 0.8, -10.0],
                radius: 0.8,
                color: srgb_to_rgb(vector![0.5, 1.0, 0.5]),
            }),
            Box::new(Sphere {
                center: point![4.5, 2.1, -10.0],
                radius: 0.5,
                color: srgb_to_rgb(vector![0.5, 1.0, 0.5]),
            }),
            Box::new(Floor { y: 0.0 }),
        ],
    };

    let camera = Camera::new(
        point![0.0, 1.5, -1.0],
        point![0.0, 1.0, -5.0],
        90.0,
        2.0,
        aspect_ratio,
    );

    let img: RgbImage = render(&config, &scene, &camera);

    img.save("output.png").unwrap();
}
