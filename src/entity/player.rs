use macroquad::prelude::*;
use crate::tilemap::Tilemap;
use macroquad::texture::Texture2D;
use crate::{DEBUG};
const SPAWN_ID: u32 = 507;
//const BLOCKING_IDS: [u32;1] = [520];
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
        let id_center = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position()+vec2(4.0,4.0));

        self.collide_color = SKYBLUE;

        let delta = get_frame_time();
        let (new_x,new_y,walking) = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)  {
            if can_walk_up(vec2(self.position.x(), self.position.y() - MOVING_SPEED*delta),tilemap){
                (self.position.x(), self.position.y() - MOVING_SPEED*delta,true)
            }else{
                self.collide_color = GOLD;
                (self.position.x(), self.position.y(),false)
            }
        }else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            if can_walk_down(vec2(self.position.x(), self.position.y() + MOVING_SPEED*delta), tilemap){
                (self.position.x(), self.position.y() + MOVING_SPEED*delta,true)
            }else{
                self.collide_color = GOLD;
                (self.position.x(), self.position.y(),false)
            }
        }else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            if can_walk_left(vec2(self.position.x() - MOVING_SPEED*delta, self.position.y()),tilemap){
                (self.position.x() - MOVING_SPEED*delta, self.position.y() ,true)
            }else{
                self.collide_color = GOLD;
                (self.position.x(), self.position.y() ,false)
            }
        }else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            if can_walk_right(vec2(self.position.x() + MOVING_SPEED*delta, self.position.y()),tilemap){
                (self.position.x() + MOVING_SPEED*delta, self.position.y() ,true)
            }else{
                self.collide_color = GOLD;
                (self.position.x(), self.position.y() ,false)
            }
        }else {
            self.position().round();
            (self.position.x(), self.position.y(), false)
        };

        if walking {
            self.position.set_x(new_x);
            self.position.set_y(new_y);
        }
    }
    pub fn position_center(&self) -> Vec2{
        vec2(self.position().x()-4.0, self.position().y()-4.0)
    }

    pub fn position(&self) -> Vec2{
        self.position.round()
        //vec2((((self.position.x()*10.0) as i32)/10) as f32,(((self.position.y()*10.0) as i32)/10) as f32)
        //vec2(self.position.x() - self.position.x() % 1.0, self.position.y() - self.position.y() % 1.0)
    }

    pub fn draw_map(&self, texture: Texture2D){
        draw_texture_ex(texture, self.position().x(), self.position().y(), WHITE,DrawTextureParams {
            source: self.source,
            ..Default::default()
        });
        if DEBUG{
            draw_circle(self.position.x().round(), self.position.y().round(),0.5, RED);
            draw_rectangle_lines(self.position().x(),self.position().y(),8.0,8.0,0.1,self.collide_color);
        }
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(0.0,0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(0.0,8.0));
    match id{
        Some(i) => {
            if i == 520 {
                return false;
            }
        },
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i == 520{
                return false;
            }
        }
        _ => {}
    }
    true
}

fn can_walk_right(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(8.0,0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(8.0,8.0));
    match id{
        Some(i) => {
            if i == 520 {
                return false;
            }
        },
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i == 520{
                return false;
            }
        }
        _ => {}
    }
    true
}

fn can_walk_up(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(0.0,0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(8.0,0.0));
    println!("{:?}:{:?}",id,id2);
    match id{
        Some(i) => {
            if i == 520 {
                return false;
            }
        },
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i == 520{
                return false;
            }
        }
        _ => {}
    }
    true
}

fn can_walk_down(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(0.0,8.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(8.0,8.0));
    println!("{:?}:{:?}",id,id2);
    match id{
        Some(i) => {
            if i == 520 {
                return false;
            }
        },
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i == 520{
                return false;
            }
        }
        _ => {}
    }
    true
}