use image::RgbImage;
use nalgebra::point;
use raytracer::camera::Camera;
use raytracer::common::RayTracable;
use raytracer::render::render;
use raytracer::render::RenderConfig;
use raytracer::scene::Floor;
use raytracer::scene::SceneList;
use raytracer::scene::Sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 1920;
    let config = RenderConfig {
        width: width,
        height: ((width as f64) / aspect_ratio).round() as u32,
        aspect_ratio: aspect_ratio,
        samples_per_pixel: 2000,
        max_depth: 50,
        tile_size: 16,
    };

    let scene: Box<dyn RayTracable> = Box::new(SceneList {
        objects: vec![
            Box::new(Sphere {
                center: point![0.0, 1.0, -5.0],
                radius: 1.0,
            }),
            Box::new(Sphere {
                center: point![-1.5, 1.0, -5.0],
                radius: 0.5,
            }),
            Box::new(Floor { y: 0.0 }),
        ],
    });

    let camera = Camera::new(
        point![0.0, 1.0, 0.0],
        point![0.0, 0.0, -5.0],
        90.0,
        2.0,
        aspect_ratio,
    );

    let img: RgbImage = render(&config, &scene, &camera);

    img.save("output.png").unwrap();
}
