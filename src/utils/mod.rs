use macroquad::prelude::*;

mod timer;
pub(crate) mod vecgrid;
pub(crate) mod tween;

#[allow(dead_code)]
pub fn clamp(num: f32, min: f32, max: f32) -> f32 {
    max.min(num).max(min)
}
