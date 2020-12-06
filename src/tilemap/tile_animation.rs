use crate::tilemap::Tilemap;
use macroquad::prelude::{get_frame_time, Rect};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TileAnimation {
    frames: Vec<Rect>,
    frame_length: Duration,
    tile_durations: Vec<Duration>,
    current_frame: usize,
    timer: Duration,
    pub repeating: bool,
}

#[allow(dead_code)]
impl TileAnimation {
    pub fn new(tilemap: &Tilemap, tile_ids: &[u32], mut tile_durations: Vec<Duration>) -> Self {
        if tile_ids.len() != tile_durations.len() {
            let duration = tile_durations[0];
            for _i in tile_durations.len()..tile_ids.len() {
                tile_durations.push(duration);
            }
        }
        TileAnimation {
            frames: tilemap.get_frames_from_ids(tile_ids),
            frame_length: tile_durations[0],
            tile_durations,
            current_frame: 0,
            timer: Duration::from_secs(0),
            repeating: true,
        }
    }

    pub fn once(tilemap: &Tilemap, tile_ids: &[u32], tile_durations: Vec<Duration>) -> Self {
        TileAnimation {
            frames: tilemap.get_frames_from_ids(tile_ids),
            frame_length: tile_durations[0],
            tile_durations,
            current_frame: 0,
            timer: Duration::from_secs(0),
            repeating: false,
        }
    }

    pub fn advance(&mut self) {
        self.advance_by(Duration::from_secs_f32(get_frame_time()));
    }

    pub fn advance_by(&mut self, duration: Duration) {
        self.timer += duration;

        let frames_remaining = self.current_frame < self.frames.len() - 1;
        if frames_remaining || self.repeating {
            while self.timer >= self.frame_length {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
                self.timer -= self.frame_length;
                self.frame_length = self.tile_durations[self.current_frame];
            }
        } else if self.timer > self.frame_length {
            self.timer = self.frame_length;
        }
    }

    pub fn restart(&mut self) {
        self.current_frame = 0;
        self.timer = Duration::from_secs(0);
    }

    pub fn finish(&mut self) -> bool {
        if !self.repeating {
            return self.current_frame == self.frames.len() - 1
                || self.current_frame == 0
                || self.current_frame == (self.frames.len() - 1) / 2;
        }
        false
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
    }

    pub fn source(&self) -> Option<Rect> {
        Some(self.frames[self.current_frame])
    }
}
