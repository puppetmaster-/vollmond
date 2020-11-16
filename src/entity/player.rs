use macroquad::prelude::*;
use crate::tilemap::Tilemap;
use crate::utils::clamp;
use macroquad::texture::Texture2D;
use crate::{PIXEL_ZOOM, DEBUG};

const SPAWN_ID: u32 = 507;
const DISTANCE: f32 = PIXEL_ZOOM;
const BLOCKING_IDS: [u32;1] = [520];
const MOVING_SPEED: f32 = 24.0;

pub struct Player{
    pub position: Vec2,
    source: Option<Rect>,
    collide_color: Color,
}

impl Player {
    pub fn new(tilemap: &Tilemap) -> Self {
        let pos = tilemap.get_all_position_from_id(tilemap.get_layer_id("logic"),SPAWN_ID)[0];
        Self {
            position: pos,
            source: Some(tilemap.get_clip_from_id(SPAWN_ID)),
            collide_color: SKYBLUE,
        }
    }
    pub fn update_map(&mut self, tilemap: &Tilemap){
        let id_center = tilemap.get_id_at(tilemap.get_layer_id("map"), self.position.x()+4.0, self.position.y()+4.0);
        if id_center.is_some(){
            self.collide_color = GOLD;
        }else{
            self.collide_color = SKYBLUE;
        }
        let delta = get_frame_time();
        let (new_x,new_y,walking) = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)  {
            (self.position.x(), self.position.y() - MOVING_SPEED*delta,true)
        }else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            (self.position.x(), self.position.y() + MOVING_SPEED*delta,true)
        }else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            (self.position.x() - MOVING_SPEED*delta, self.position.y() ,true)
        }else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            (self.position.x() + MOVING_SPEED*delta, self.position.y() ,true)
        }else {
            (self.position.x(), self.position.y(), false)
        };

        if walking {
            self.position.set_x(new_x);
            self.position.set_y(new_y);
        }
    }
    pub fn position(&self) -> Vec2{
        vec2(self.position.x() - self.position.x() % 1.0, self.position.y() - self.position.y() % 1.0)
    }

    pub fn draw_map(&self, texture: Texture2D){
        draw_texture_ex(texture, self.position().x(), self.position().y(), WHITE,DrawTextureParams {
            source: self.source,
            ..Default::default()
        });
        if DEBUG{
            draw_rectangle_lines(self.position().x(),self.position().y(),8.0,8.0,1.0,self.collide_color);
        }
    }
}