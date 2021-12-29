use crate::common::Float;
use crate::common::Vector;
use image::Rgb;

pub fn rgb_to_srgb(colour: Vector) -> Rgb<u8> {
    Rgb([
        rgb_to_srgb_channel(colour.x),
        rgb_to_srgb_channel(colour.y),
        rgb_to_srgb_channel(colour.z),
    ])
}

fn rgb_to_srgb_channel(value: Float) -> u8 {
    let srgb = if value <= 0.0031308 {
        12.92 * value
    } else {
        1.055 * value.powf(1.0 / 2.4) - 0.055
    };
    if srgb <= 0.0 {
        0
    } else if srgb >= 1.0 {
        255
    } else {
        (255.0 * srgb).round() as u8
    }
}
