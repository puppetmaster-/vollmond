#[doc(no_inline)]
pub use keyframe::*;
pub use keyframe::functions::*;
pub use keyframe_derive::*;

use crate::utils::timer::Timer;
use std::time::Duration;

pub struct Tween {
    timer: Timer,
    sequence: Option<AnimationSequence<f32>>,
    repeat: bool,
}

#[allow(dead_code)]
impl Tween{
    pub fn from_keyframes(keyframes: Vec<Keyframe<f32>>,start_at: u64, duration_sec: u64, repeat: bool) -> Tween{
        let sequence = AnimationSequence::from(keyframes);
        let mut timer = Timer::new_sec(duration_sec);
        timer.advance_by((start_at / 1000) as f64);
        Self{
            timer,
            sequence: Some(sequence),
            repeat,
        }
    }

    pub fn restart(&mut self){
        self.timer.restart();
    }

    pub fn finished(&self) -> bool{
        if let Some(s) = self.sequence.as_ref(){
            s.finished()
        }else{
            false
        }
    }

    pub fn update(&mut self){
        if let Some(s) = self.sequence.as_mut() {
            s.advance_to(self.timer.value() as f64);
        }

        if self.timer.finished() && self.repeat{
            self.timer.restart();
        }
    }

    pub fn value(&self) -> f32{
        if let Some(s) = self.sequence.as_ref(){
            s.now()
        }else{
            1.0
        }
    }
}