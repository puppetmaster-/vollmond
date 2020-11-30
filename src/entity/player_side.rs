use macroquad::prelude::*;
use std::time::Duration;
use crate::tilemap::Tilemap;
use macroquad::texture::Texture2D;
use crate::{DEBUG};
use crate::scene::game::GameState;
use crate::tilemap::tile_animation::TileAnimation;
use std::collections::HashMap;
use crate::utils::timer::Timer;

pub const SPAWN_ID: u32 = 507;
const EXIT: u32 = 510;

const JUMP_UP_FACTOR: f32 = 2.5;
const JUMP_DOWN_FACTOR: f32 = 2.0;
const JUMP_UP_CURVE: [f32;12] = [8.0, 15.0, 13.0, 8.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 1.0, 1.0];
const JUMP_DOWN_CURVE: [f32;6] = [0.0, 1.0, 1.0, 2.0, 3.0, 5.0];

const AIR_TIMER_MAX: usize = 5;

const MOVE_FACTOR: f32 = 4.0;
const MOVE_SPEED_CURVE: [f32;8] = [1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0];
const BREAK_SPEED_CURVE: [f32;8] = [21.0, 13.0, 8.0, 5.0, 3.0, 2.0, 1.0, 1.0];

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State{
    FLOOR,
    IDLE,
    RUN,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum JumpState{
    NOT,
    JUMP,
    AIR,
    DOWN,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AnimationState{
    RUNLEFT,
    RUNRIGHT,
    STANDLEFT,
    STANDRIGHT,
}

pub struct PlayerSide {
    pub ingredients: u8,
    moving_speed: f32,
    moving_timer: usize,
    break_timer: usize,
    air_timer: usize,
    jump_up_timer: usize,
    jump_down_timer: usize,
    direction: Vec2,
    pub position: Vec2,
    collide_color: Color,
    spritesheet: Texture2D,
    start_position: Vec2,
    need_reset: bool,
    jump_timer: u32,
    state: State,
    jump_state: JumpState,
    animation_state: AnimationState,
    animations: HashMap<AnimationState, TileAnimation>,
    start_timer: Timer,
}

impl PlayerSide {
    pub fn new() -> Self {
        let spritesheet= get_player_spritesheet();
        let animations = get_animations();
        Self {
            ingredients: 0,
            moving_speed: 0.0,
            moving_timer: 0,
            break_timer: 0,
            air_timer: 0,
            jump_up_timer: 0,
            jump_down_timer: 0,
            direction: Vec2::zero(),
            position: Vec2::zero(),
            collide_color: SKYBLUE,
            spritesheet,
            start_position: Vec2::zero(),
            need_reset: true,
            jump_timer: 0,
            state: State::FLOOR,
            jump_state: JumpState::NOT,
            animation_state: AnimationState::STANDRIGHT,
            animations,
            start_timer: Timer::new(500),
        }
    }
    pub fn update(&mut self, tilemap: &Tilemap) -> Option<GameState>{

        if self.need_reset{
            self.state = State::IDLE;
            self.jump_timer = 0;
            self.moving_timer = 0;
            self.break_timer = 0;
            self.position = self.start_position;
            self.need_reset = false;
            self.start_timer.restart();
            self.animation_state = AnimationState::STANDRIGHT;
            for (_, a) in self.animations.iter_mut() {
                a.reset();
            }
        }
        if self.start_timer.finished() {
            self.animations.get_mut(&self.animation_state).unwrap().advance();

            let id_center = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position() + vec2(4.0, 4.0));

            self.collide_color = SKYBLUE;

            let delta = get_frame_time();
            let mut new_x = self.position.x();
            let mut new_y = self.position.y();

            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                let distance = 4.0 * MOVE_SPEED_CURVE[self.moving_timer] * delta;
                if can_walk_left(vec2(self.position.x() - distance, self.position.y()), tilemap) {
                    if self.animation_state != AnimationState::RUNLEFT{
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::RUNLEFT;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    self.state = State::RUN;
                    self.direction = vec2(-1.0,0.0);
                    new_x = self.position.x() - distance;
                    if self.moving_timer < MOVE_SPEED_CURVE.len()-1{
                        self.moving_timer +=1;
                    }
                    self.break_timer = 0;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                let distance = MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta;
                if can_walk_right(vec2(self.position.x() + distance, self.position.y()), tilemap) {
                    if self.animation_state != AnimationState::RUNRIGHT{
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::RUNRIGHT;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    self.state = State::RUN;
                    self.direction = vec2(1.0,0.0);
                    new_x = self.position.x() + distance;
                    if self.moving_timer < MOVE_SPEED_CURVE.len()-1{
                        self.moving_timer +=1;
                    }
                    self.break_timer = 0;
                } else {
                    self.collide_color = GOLD;
                }
            } else {
                self.moving_timer = 0;
                if self.break_timer < BREAK_SPEED_CURVE.len()-1 {
                    println!("break {}",self.break_timer);
                    //TODO fix slide into wall
                    new_x = self.position.x() + self.direction.x() * (MOVE_FACTOR + 2.0) * BREAK_SPEED_CURVE[self.break_timer] * delta;
                    self.break_timer +=1;
                    println!("new_x: {}",new_x);
                }
                match self.animation_state {
                    AnimationState::RUNLEFT => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.animation_state = AnimationState::STANDLEFT;
                        }
                    }
                    AnimationState::RUNRIGHT => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.animation_state = AnimationState::STANDRIGHT;
                        }
                    }
                    _ => {}
                }
            };
            // jump
            if is_key_down(KeyCode::Space) && (self.jump_state == JumpState::JUMP || self.jump_state == JumpState::NOT)  {
                println!("jump up = {}",self.jump_up_timer);
                if self.jump_up_timer < JUMP_UP_CURVE.len()-1 {
                    self.jump_state = JumpState::JUMP;
                    self.jump_up_timer += 1;
                    new_y = self.position.y() - JUMP_UP_FACTOR + JUMP_UP_CURVE[self.jump_up_timer] * delta;
                    new_x = new_x + self.direction.x() * 0.2;
                }else{
                    self.jump_state = JumpState::AIR;
                }
            }

            //stop jumping
            if !is_key_down(KeyCode::Space) && self.jump_state == JumpState::JUMP {
                self.jump_state = JumpState::AIR;
                self.jump_up_timer = 0;
            }

            if self.jump_state == JumpState::AIR{
                if self.air_timer < AIR_TIMER_MAX{
                    self.air_timer +=1;
                }else{
                    self.air_timer = 0;
                    self.jump_state = JumpState::DOWN;
                }
            }

            if self.jump_state == JumpState::DOWN || self.jump_state == JumpState::NOT {
                if can_walk_down(vec2(self.position.x(), self.position.y()), tilemap) {
                    if self.jump_down_timer < JUMP_DOWN_CURVE.len()-1 {
                        self.jump_down_timer += 1;
                    }
                    new_y = self.position.y() + JUMP_DOWN_FACTOR + JUMP_DOWN_CURVE[self.jump_down_timer]* delta;
                    new_x = new_x + self.direction.x()  * 0.2;
                    self.jump_state = JumpState::DOWN;
                }else{
                    self.jump_down_timer = 0;
                    self.jump_up_timer = 0;
                    self.jump_state =JumpState::NOT;
                    self.state = State::FLOOR;
                }
            }

            if self.position.x() == new_x && self.position.y() == new_y {
                if self.position.y() % 8.0 > 0.0{
                    self.position = vec2(self.position.x(),self.position.y()-self.position.y() % 8.0);
                }
                self.direction = vec2(0.0,0.0);
                self.position.set_x(new_x);
            }else{
                println!("set new_x");
                self.position.set_x(new_x);
                self.position.set_y(new_y);
            }

            if id_center == Some(EXIT) {
                self.need_reset = true;
                Some(GameState::MAP)
            } else {
                None
            }
        }else{
            None
        }
    }

    pub fn position(&self) -> Vec2{
        self.position.round()
    }

    pub fn draw(&self){
        draw_texture_ex(self.spritesheet, self.position().x()-2.0, self.position().y(), WHITE,DrawTextureParams {
            source: self.animations.get(&self.animation_state).unwrap().source(),
            ..Default::default()
        });
        if DEBUG{
            draw_circle(self.position.x().round(), self.position.y().round(),0.5, RED);
            draw_rectangle_lines(self.position().x(),self.position().y(),16.0,16.0,0.1,self.collide_color);
            draw_text(&format!("{:?}",&self.state),400.0,5.0,14.0,WHITE);
            draw_circle((self.position+vec2(0.0,8.0)).x(),(self.position+vec2(0.0,8.0)).y(),0.5, GREEN);
            draw_circle((self.position+vec2(8.0,0.0)).x(),(self.position+vec2(8.0,0.0)).y(),0.5, GREEN);
            draw_circle((self.position+vec2(8.0,8.0)).x(),(self.position+vec2(8.0,8.0)).y(),0.5, GREEN);
            draw_circle((self.position+vec2(0.0,16.0)).x(),(self.position+vec2(0.0,16.0)).y(),0.5, GREEN);
            draw_circle((self.position+vec2(8.0,16.0)).x(),(self.position+vec2(8.0,16.0)).y(),0.5, GREEN);
        }
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap) -> bool{
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(0.0,0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(0.0,8.0));
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
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(8.0,0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(8.0,8.0));
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
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(0.0,0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(8.0,0.0));
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
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(0.0,16.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(8.0,16.0));
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

fn get_animations() -> HashMap<AnimationState,TileAnimation>{
    let player_tilemap = Tilemap::new(Rect::new(0.0,0.0,160.0,128.0),16,16,10,8);
    let mut hashmap = HashMap::new();
    hashmap.insert(AnimationState::RUNRIGHT,TileAnimation::new(&player_tilemap,&[0,1,2,3,4,5,6,7],vec![Duration::from_millis(80)]));
    hashmap.insert(AnimationState::RUNLEFT,TileAnimation::new(&player_tilemap,&[10,11,12,13,14,15,16,17],vec![Duration::from_millis(80)]));
    hashmap.insert(AnimationState::STANDRIGHT,TileAnimation::new(&player_tilemap,&[0,20],vec![Duration::from_millis(500)]));
    hashmap.insert(AnimationState::STANDLEFT,TileAnimation::new(&player_tilemap,&[10,30],vec![Duration::from_millis(500)]));
    hashmap
}

fn get_player_spritesheet() -> Texture2D{
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/player.png"), None);
    let spritesheet: Texture2D = load_texture_from_image(&image);
    set_texture_filter(spritesheet,FilterMode::Nearest);
    spritesheet
}