use macroquad::prelude::*;

pub(crate) mod timer;
pub(crate) mod tween;
pub(crate) mod vecgrid;

#[allow(dead_code)]
pub fn clamp(num: f32, min: f32, max: f32) -> f32 {
    max.min(num).max(min)
}

#[allow(dead_code)]
pub fn rgba8_color(r: u8, g: u8, b: u8, a: u8) -> Color {
    let r = f32::from(r) / 255.0;
    let g = f32::from(g) / 255.0;
    let b = f32::from(b) / 255.0;
    let a = f32::from(a) / 255.0;

    Color { r, g, b, a }
}
