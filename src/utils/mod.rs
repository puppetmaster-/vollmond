use macroquad::prelude::*;
use crate::MAP_ZOOM;

mod timer;
pub(crate) mod vecgrid;
pub(crate) mod tween;

pub fn clamp(num: f32, min: f32, max: f32) -> f32 {
    max.min(num).max(min)
}
