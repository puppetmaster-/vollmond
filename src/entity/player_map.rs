use macroquad::prelude::*;
use crate::tilemap::Tilemap;
use macroquad::texture::Texture2D;
use crate::{DEBUG};
use crate::scene::game::GameState;

const SPAWN_ID: u32 = 507;
//const BLOCKING_IDS: [u32;1] = [520];
const MOVING_SPEED: f32 = 0.8;
const HAUS1: u32 = 508;
const CEMETRY: u32 = 509;
const GROUND_GRASS: u32 = 533;
const GROUND_ICE: u32 = 535;
const GROUND_ROCK: u32 = 534;
const GROUND_SAND: u32 = 536;
const GROUND_SWAMP: u32 = 537;
const GROUND_STREET: u32 = 540;

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
        let ground = tilemap.get_id_at_position(tilemap.get_layer_id("background"), self.position()+vec2(4.0,4.0));

        let moving_speed_factor = match ground {
            Some(id) => {
                match id {
                    GROUND_GRASS => 0.9,
                    GROUND_ROCK => 0.7,
                    GROUND_SAND => 0.6,
                    GROUND_ICE => 1.1,
                    GROUND_SWAMP => 0.4,
                    _ => 1.0
                }
            }
            _ => 1.0
        };

        //let delta = get_frame_time(); //todo subpixel and speed problem
        self.collide_color = SKYBLUE;
        let velocity = MOVING_SPEED * moving_speed_factor;
        let mut new_x = self.position.x();
        let mut new_y = self.position.y();

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)  {
            if can_walk_up(vec2(self.position.x(), self.position.y() - velocity),tilemap){
                new_y = self.position.y() - velocity;
            }else{
                self.collide_color = GOLD;
            }
        }else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            if can_walk_down(vec2(self.position.x(), self.position.y() + velocity), tilemap){
                new_y = self.position.y() + velocity;
            }else{
                self.collide_color = GOLD;
            }
        }else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            if can_walk_left(vec2(self.position.x() - velocity, self.position.y()),tilemap){
                new_x = self.position.x() - velocity;
            }else{
                self.collide_color = GOLD;
            }
        }else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            if can_walk_right(vec2(self.position.x() + velocity, self.position.y()),tilemap){
                new_x = self.position.x() + velocity;
            }else{
                self.collide_color = GOLD;
            }
        }else {


        };

        if new_x == self.position.x() {
            if self.position.x() % 1.0 > 0.0 {
                self.position = vec2(self.position.x() - self.position.x() % 1.0, self.position.y());
            }
            self.position.set_y(new_y);
        }else if new_y == self.position.y(){
            if self.position.y() % 1.0 > 0.0 {
                self.position = vec2(self.position.x(), self.position.y() - self.position.y() % 1.0);
            }
            self.position.set_x(new_x);
        }else{
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
            self.last_id = None;
            None
        }else{
            None
        }
    }

    pub fn position(&self) -> Vec2{
        self.position.round()
    }

    pub fn draw(&self, texture: Texture2D){
        draw_texture_ex(texture, self.position().x(), self.position().y(), WHITE,DrawTextureParams {
            source: self.source,
            ..Default::default()
        });
        if DEBUG{
            draw_circle(self.position.x().round(), self.position.y().round(),0.5, RED);
            draw_rectangle_lines(self.position().x(),self.position().y(),7.0,7.0,0.1,self.collide_color);
            draw_circle((self.position()+vec2(4.0,4.0)).x() , (self.position()+vec2(4.0,4.0)).y(), 0.5, RED);
            draw_circle((self.position+vec2(2.0,3.0)).x(),(self.position+vec2(2.0,3.0)).y(),0.5, GREEN);
            draw_circle((self.position+vec2(2.0,8.0)).x(),(self.position+vec2(2.0,8.0)).y(),0.5, GREEN);
            draw_circle((self.position+vec2(6.0,8.0)).x(),(self.position+vec2(6.0,8.0)).y(),0.5, GREEN);
            draw_circle((self.position+vec2(6.0,3.0)).x(),(self.position+vec2(6.0,3.0)).y(),0.5, GREEN);
        }
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(2.0,3.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(2.0,7.0));
    if let Some(i) = id {
        if i >= 520 && i <= 532 {
            return false;
        }
    }

    if let Some(i) = id2 {
        if i >= 520 && i <= 532 {
            return false;
        }
    }
    true
}

fn can_walk_right(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(5.5,3.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(5.5,7.0));
    match id{
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        },
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i >= 520 && i <= 532{
                return false;
            }
        }
        _ => {}
    }
    true
}

fn can_walk_up(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(2.0,3.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(5.0,3.0));
    match id{
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        },
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i >= 520 && i <= 532{
                return false;
            }
        }
        _ => {}
    }
    true
}

fn can_walk_down(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(2.0,7.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position+vec2(5.0,7.0));
    match id{
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        },
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i >= 520 && i <= 532{
                return false;
            }
        }
        _ => {}
    }
    true
}