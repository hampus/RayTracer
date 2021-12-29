use crate::camera::Camera;
use crate::common::Direction;
use crate::common::Float;
use crate::common::Ray;
use crate::common::RayTracable;
use crate::common::Vector;
use crate::common::INFINITY;
use crate::srgb::rgb_to_srgb;
use crate::srgb::srgb_to_rgb;
use image::{GenericImage, RgbImage};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use nalgebra::Unit;
use nalgebra::{point, vector, Point2, Vector2};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::cmp;

pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: Float,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub tile_size: u32,
}

pub fn render(config: &RenderConfig, scene: &dyn RayTracable, camera: &Camera) -> RgbImage {
    let tiles = generate_shuffled_tiles(config);
    println!("Number of tiles: {}", tiles.len());

    let pb = ProgressBar::new(tiles.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar().template(
            "[{elapsed_precise} of {duration_precise}] {spinner} {wide_bar} {percent}% [ETA: {eta}] {msg}",
        ),
    );
    pb.tick();

    let rendered_tiles: Vec<(RenderTile, RgbImage)> = tiles
        .into_par_iter()
        .progress_with(pb)
        .map(|tile| render_tile(tile, config, scene, camera))
        .collect();

    let mut img = RgbImage::new(config.width, config.height);
    for (tile, tile_img) in rendered_tiles {
        img.copy_from(&tile_img, tile.offset.x, tile.offset.y)
            .unwrap();
    }
    img
}

fn render_tile(
    tile: RenderTile,
    config: &RenderConfig,
    scene: &dyn RayTracable,
    camera: &Camera,
) -> (RenderTile, RgbImage) {
    let mut rng = thread_rng();
    let mut img = RgbImage::new(tile.size.x, tile.size.y);
    for y in 0..tile.size.y {
        for x in 0..tile.size.x {
            let mut colour = vector![0.0, 0.0, 0.0];
            for _ in 0..config.samples_per_pixel {
                let sx: Float = rng.gen::<Float>() - 0.5;
                let sy: Float = rng.gen::<Float>() - 0.5;
                let uv = point![
                    (((tile.offset.x + x) as Float + sx) / config.width as Float - 0.5) * 2.0,
                    (0.5 - ((tile.offset.y + y) as Float + sy) / config.height as Float) * 2.0
                ];
                colour += render_sample(uv, scene, camera, config.max_depth);
            }
            colour /= config.samples_per_pixel as Float;
            img.put_pixel(x, y, rgb_to_srgb(colour));
        }
    }
    (tile, img)
}

fn render_sample(
    uv: Point2<Float>,
    scene: &dyn RayTracable,
    camera: &Camera,
    max_depth: u32,
) -> Vector {
    let ray = camera.generate_ray(uv);
    render_ray(&ray, scene, 0.001, INFINITY, max_depth)
}

fn render_ray(
    ray: &Ray,
    scene: &dyn RayTracable,
    min_dist: Float,
    max_dist: Float,
    max_depth: u32,
) -> Vector {
    if max_depth == 0 {
        vector![0.0, 0.0, 0.0]
    } else if let Some(intersection) = scene.trace_ray(ray, min_dist, max_dist) {
        let new_direction = random_direction_on_hemisphere_cosine_weighted(&intersection.normal);
        let new_ray = Ray {
            origin: intersection.position,
            direction: new_direction,
        };
        0.5 * render_ray(&new_ray, scene, min_dist, max_dist, max_depth - 1)
    } else {
        srgb_to_rgb(vector![0.5, 0.55, 1.0])
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

#[derive(Debug, PartialEq)]
struct RenderTile {
    offset: Point2<u32>,
    size: Vector2<u32>,
}

fn integer_div_round_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

fn generate_shuffled_tiles(config: &RenderConfig) -> Vec<RenderTile> {
    let mut rng = thread_rng();
    let mut tiles = generate_tiles(config.width, config.height, config.tile_size);
    tiles.shuffle(&mut rng);
    tiles
}

fn generate_tiles(width: u32, height: u32, tile_size: u32) -> Vec<RenderTile> {
    let tiles_x = integer_div_round_up(width, tile_size);
    let tiles_y = integer_div_round_up(height, tile_size);
    let size = tile_size;
    (0..tiles_y)
        .flat_map(|y| {
            (0..tiles_x).map(move |x| RenderTile {
                offset: point![x * size, y * size],
                size: vector![
                    cmp::min(size, width - x * size),
                    cmp::min(size, height - y * size)
                ],
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_div_round_up_returns_correct_values() {
        assert_eq!(2, integer_div_round_up(10, 7));
        assert_eq!(2, integer_div_round_up(10, 9));
        assert_eq!(1, integer_div_round_up(10, 10));
        assert_eq!(1, integer_div_round_up(10, 11));
        assert_eq!(11, integer_div_round_up(99, 9));
        assert_eq!(12, integer_div_round_up(100, 9));
    }

    #[test]
    fn generated_tiles_for_small_example_works() {
        let tiles = generate_tiles(98, 50, 50);
        assert_eq!(
            tiles,
            vec![
                RenderTile {
                    offset: point![0, 0],
                    size: vector![50, 50]
                },
                RenderTile {
                    offset: point![50, 0],
                    size: vector![48, 50]
                }
            ]
        );
    }

    #[test]
    fn generated_tiles_supports_tiles_in_both_dimensions() {
        let tiles = generate_tiles(103, 98, 50);
        assert_eq!(
            tiles,
            vec![
                RenderTile {
                    offset: point![0, 0],
                    size: vector![50, 50]
                },
                RenderTile {
                    offset: point![50, 0],
                    size: vector![50, 50]
                },
                RenderTile {
                    offset: point![100, 0],
                    size: vector![3, 50]
                },
                RenderTile {
                    offset: point![0, 50],
                    size: vector![50, 48]
                },
                RenderTile {
                    offset: point![50, 50],
                    size: vector![50, 48]
                },
                RenderTile {
                    offset: point![100, 50],
                    size: vector![3, 48]
                },
            ]
        );
    }
}
