use std::ops::Add;
use miniquad::date;


#[derive(Debug, Clone, PartialEq)]
pub struct Timer{
    duration: f64,
    start_time: f64,
}

#[allow(dead_code)]
impl Timer{
    pub fn new(duration_millis: u64)-> Timer{
        Timer{
            duration: duration_millis as f64,
            start_time: date::now() * 1000.0,
        }
    }
    pub fn new_sec(duration_sec: u64)-> Timer{
        Timer::new(duration_sec * 1000)
    }

    pub fn advance_by(&mut self, duration_millis: f64){
        self.start_time = self.start_time.add(duration_millis);
    }

    pub fn advance_by_sec(&mut self, duration_sec: f64){
        self.start_time = self.start_time.add(duration_sec * 1000.0);
    }

    pub fn finished(&self) -> bool{
        let current_time = date::now() * 1000.0;
        let elapsed = current_time - self.start_time;
        elapsed >= self.duration
    }

    #[allow(dead_code)]
    pub fn set_duration(&mut self, duration_millis: u64){
        self.duration = duration_millis as f64;
    }

    pub fn restart(&mut self){
        self.start_time = date::now() * 1000.0;
    }

    pub fn value(&self)-> f32{
        let current_time = date::now() * 1000.0;
        let elapsed = current_time - self.start_time;
        if elapsed < self.duration{
            1.0 * (100.0 / self.duration as f32 * elapsed as f32) / 100.0
        }else{
            1.0
        }
    }
}