mod timer;
pub(crate) mod vecgrid;

pub fn clamp(num: f32, min: f32, max: f32) -> f32 {
    max.min(num).max(min)
    /*
    assert!(min <= max);
    let mut x = num;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
     */
}