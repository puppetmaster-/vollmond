use crate::tilemap::Tilemap;
use std::time::Duration;
use macroquad::prelude::{get_frame_time, Texture2D, draw_texture_ex, DrawTextureParams, WHITE, Rect};

pub struct TileAnimation{
    frames: Vec<Rect>,
    frame_length: Duration,
    tile_durations: Vec<Duration>,
    current_frame: usize,
    timer: Duration,
    repeating: bool,
}

impl TileAnimation {
    pub fn new(tilemap: &Tilemap, tile_ids: &[u32], mut tile_durations: Vec<Duration>) -> Self{
        if tile_ids.len() != tile_durations.len(){
            let duration = tile_durations[0];
            for _i in tile_durations.len()..tile_ids.len(){
                tile_durations.push(duration);
            }
        }
        TileAnimation{
            frames: tilemap.get_frames_from_ids(tile_ids),
            frame_length: tile_durations[0],
            tile_durations,
            current_frame: 0,
            timer: Duration::from_secs(0),
            repeating: true,
        }
    }

    pub fn once(tilemap: &Tilemap, tile_ids: &[u32],tile_durations: Vec<Duration>) -> Self{
        TileAnimation{
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

    pub fn source(&self) -> Option<Rect>{
        Some(self.frames[self.current_frame])
    }

    /*
    pub fn draw<P>(&mut self, texture: Texture2D, params: P){
        where
        P: Into<DrawTextureParams>,
        {
            let mut params = params.into();
            params.source = Some(self.frames[self.current_frame]);
            draw_texture_ex(texture, self.position.x() , self.position.y(), WHITE,DrawTextureParams {
                source: Some(self.frames[self.current_frame]),
                ..Default::default()
            });
        }
        let mut params = params.into();
        draw_texture_ex(texture, self.position.x() , self.position.y(), WHITE,DrawTextureParams {
            source: self.source,
            ..Default::default()
        });
    }

    pub fn draw<P>(&mut self, ctx: &mut Context, texture: &Texture, params: P)
        where
            P: Into<DrawParams>,
    {
        let mut params = params.into();
        params.clip = Some(self.frames[self.current_frame]);
        graphics::draw(ctx, texture, params)
    }
     */
}