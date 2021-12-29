use crate::common::Float;
use crate::common::Vector;
use image::Rgb;
use nalgebra::vector;

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

pub fn srgb_to_rgb(colour: Vector) -> Vector {
    vector![
        srgb_to_rgb_channel(colour.x),
        srgb_to_rgb_channel(colour.y),
        srgb_to_rgb_channel(colour.z)
    ]
}

fn srgb_to_rgb_channel(value: Float) -> Float {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::vector;

    #[test]
    fn test_linear_to_srgb_colours() {
        assert_eq!(rgb_to_srgb(vector![0.0, 0.0, 0.0]), Rgb([0, 0, 0]));
        assert_eq!(rgb_to_srgb(vector![0.0, 0.0, 1.0]), Rgb([0, 0, 255]));
        assert_eq!(rgb_to_srgb(vector![0.5, 0.5, 0.5]), Rgb([188, 188, 188]));
    }

    #[test]
    fn converts_back_to_same() {
        assert_eq!(
            srgb_to_rgb_channel(rgb_to_srgb_channel(0.0) as Float / 255.0),
            0.0
        );
        assert_eq!(
            srgb_to_rgb_channel(rgb_to_srgb_channel(1.0) as Float / 255.0),
            1.0
        );
        assert_eq!(
            srgb_to_rgb_channel(rgb_to_srgb_channel(0.5) as Float / 255.0),
            0.5
        );
        assert_eq!(
            srgb_to_rgb_channel(rgb_to_srgb_channel(0.8) as Float / 255.0),
            0.8
        );
    }
}
