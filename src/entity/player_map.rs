use macroquad::prelude::*;
use crate::tilemap::Tilemap;
use macroquad::texture::Texture2D;
use crate::{DEBUG};
use crate::scene::game::GameState;

const SPAWN_ID: u32 = 507;
//const BLOCKING_IDS: [u32;1] = [520];
const MOVING_SPEED: f32 = 30.0;
const HAUS1: u32 = 508;
const CEMETRY: u32 = 509;

pub struct PlayerMap {
    pub position: Vec2,
    source: Option<Rect>,
    collide_color: Color,
    last_id: Option<u32>,
}

impl PlayerMap {
    pub fn new(tilemap: &Tilemap) -> Self {
        let pos = tilemap.get_all_position_from_id(tilemap.get_layer_id("logic"),SPAWN_ID)[0];
        Self {
            position: pos,
            source: Some(tilemap.get_clip_from_id(SPAWN_ID)),
            collide_color: SKYBLUE,
            last_id: None
        }
    }
    pub fn update(&mut self, tilemap: &Tilemap) -> Option<GameState>{
        let id_center = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position()+vec2(4.0,4.0));

        let delta = get_frame_time();
        self.collide_color = SKYBLUE;
        let velocity = MOVING_SPEED*delta;

        let (new_x,new_y,walking) = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)  {
            if can_walk_up(vec2(self.position.x(), self.position.y() - velocity),tilemap){
                (self.position.x(), self.position.y() - velocity,true)
            }else{
                self.collide_color = GOLD;
                (self.position.x(), self.position.y(),false)
            }
        }else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            if can_walk_down(vec2(self.position.x(), self.position.y() + velocity), tilemap){
                (self.position.x(), self.position.y() + velocity,true)
            }else{
                self.collide_color = GOLD;
                (self.position.x(), self.position.y(),false)
            }
        }else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            if can_walk_left(vec2(self.position.x() - velocity, self.position.y()),tilemap){
                (self.position.x() - velocity, self.position.y() ,true)
            }else{
                self.collide_color = GOLD;
                (self.position.x(), self.position.y() ,false)
            }
        }else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            if can_walk_right(vec2(self.position.x() + velocity, self.position.y()),tilemap){
                (self.position.x() + velocity, self.position.y() ,true)
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
        if id_center == Some(HAUS1) && self.last_id != Some(HAUS1){
            self.last_id = Some(HAUS1);
            Some(GameState::HOUSE1)
        }else if id_center == Some(CEMETRY) && self.last_id != Some(CEMETRY){
            self.last_id = Some(CEMETRY);
            Some(GameState::CEMETERY)
        }else if id_center.is_none(){
            self.last_id = None
            ;
            None
        }else{
            None
        }
    }

    pub fn position(&self) -> Vec2{
        self.position.round()
        //vec2((((self.position.x()*10.0) as i32)/10) as f32,(((self.position.y()*10.0) as i32)/10) as f32)
        //vec2(self.position.x() - self.position.x() % 1.0, self.position.y() - self.position.y() % 1.0)
    }

    pub fn draw(&self, texture: Texture2D){
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

//new_position+vec2(0.0,0.0)
//new_position+vec2(0.0,8.0)
fn can_walk_at(new_position1: Vec2,new_position2: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position1);
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position2);
    if let Some(i) = id {
        if i == 520 {
            return false;
        }
    }

    if let Some(i) = id2 {
        if i == 520{
            return false;
        }
    }
    true
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(0.0,0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(0.0,8.0));
    if let Some(i) = id {
        if i == 520 {
            return false;
        }
    }

    if let Some(i) = id2 {
        if i == 520{
            return false;
        }
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