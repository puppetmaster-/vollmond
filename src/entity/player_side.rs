use macroquad::prelude::*;
use std::time::Duration;
use crate::tilemap::Tilemap;
use macroquad::texture::Texture2D;
use crate::{DEBUG};
use crate::scene::game::GameState;
use crate::tilemap::tile_animation::TileAnimation;
use std::collections::HashMap;
use crate::utils::timer::Timer;

const SPAWN_ID: u32 = 507;
const EXIT: u32 = 510;

const JUMP_SPEED_UP: f32 = 60.0;
const JUMP_SPEED_DOWN: f32 = 80.0;
const JUMP_TIMER_MAX: u32 = 20;

const MOVING_SPEED: f32 = 60.0;
const MOVING_TIMER_MAX: usize = 6;
const MOVE_SPEED_CURVE: [f32;7] = [1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0];
const BREAK_SPEED_CURVE: [f32;3] = [13.0, 5.0, 1.0];

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State{
    FLOOR,
    JUMP,
    AIR,
    IDLE,
    RUN,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AnimationState{
    RUNLEFT,
    RUNRIGHT,
    STANDLEFT,
    STANDRIGHT,
}

pub struct PlayerSide {
    moving_speed: f32,
    moving_timer: usize,
    break_timer: usize,
    direction: Vec2,
    pub position: Vec2,
    collide_color: Color,
    spritesheet: Texture2D,
    start_position: Vec2,
    need_reset: bool,
    jump_timer: u32,
    state: State,
    animation_state: AnimationState,
    animations: HashMap<AnimationState, TileAnimation>,
    start_timer: Timer,
}

impl PlayerSide {
    pub fn new(tilemap: &Tilemap) -> Self {
        let mut start_position= tilemap.get_all_position_from_id(tilemap.get_layer_id("logic"),SPAWN_ID)[0];
        start_position.set_y(start_position.y());
        let spritesheet= get_player_spritesheet();
        let animations = get_animations();
        Self {
            moving_speed: 0.0,
            moving_timer: 0,
            break_timer: 0,
            direction: Vec2::zero(),
            position: start_position,
            collide_color: SKYBLUE,
            spritesheet,
            start_position,
            need_reset: true,
            jump_timer: 0,
            state: State::FLOOR,
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
            self.break_timer = 3;
            self.position = self.start_position;
            self.need_reset = false;
            self.start_timer.restart();
            self.animation_state = AnimationState::STANDRIGHT;
            for (s, a) in self.animations.iter_mut() {
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
                    if self.moving_timer < MOVING_TIMER_MAX{
                        self.moving_timer +=1;
                    }
                    self.break_timer = 3;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                let distance = 4.0 * MOVE_SPEED_CURVE[self.moving_timer] * delta;
                if can_walk_right(vec2(self.position.x() + distance, self.position.y()), tilemap) {
                    if self.animation_state != AnimationState::RUNRIGHT{
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::RUNRIGHT;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    self.state = State::RUN;
                    self.direction = vec2(1.0,0.0);
                    new_x = self.position.x() + distance;
                    if self.moving_timer < MOVING_TIMER_MAX{
                        self.moving_timer +=1;
                    }
                    self.break_timer = 3;
                } else {
                    self.collide_color = GOLD;
                }
            } else {
                self.moving_timer = 0;
                if self.break_timer > 0{
                    self.break_timer -=1;
                    new_x = self.position.x() + self.direction.x() * 4.0 * BREAK_SPEED_CURVE[self.break_timer] * delta;
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
            if is_key_down(KeyCode::Space) && self.state != State::AIR{
                if self.jump_timer < JUMP_TIMER_MAX {
                    println!("jump");
                    self.state = State::JUMP;
                    self.jump_timer += 1;
                    new_y = self.position.y() - JUMP_SPEED_UP * delta;
                }else{
                    self.state = State::AIR;
                }
            }
            if !is_key_down(KeyCode::Space) && self.state == State::JUMP {
                println!("jump finish");
                self.state = State::AIR;
                self.jump_timer = 0;
            }
            if self.state == State::AIR || self.state == State::FLOOR || self.state == State::RUN {
                if can_walk_down(vec2(self.position.x(), self.position.y()), tilemap) {
                    println!("in air");
                    new_y = self.position.y() + JUMP_SPEED_DOWN * delta;
                    self.state = State::AIR;
                }else{
                    println!("air finish, floor now");
                    self.jump_timer = 0;
                    self.state = State::FLOOR;
                }
            }

            if self.position.x() == new_x && self.position.y() == new_y{
                println!("{}",self.position.y() % 8.0);
                if self.position.y() % 8.0 > 0.0{
                    self.position = vec2(self.position.x(),self.position.y()-self.position.y() % 8.0);
                }
                //self.position = self.position.round()
            }else{
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

    pub fn draw(&self, texture: Texture2D){
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
    draw_circle((new_position+vec2(0.0,8.0)).x(),(new_position+vec2(0.0,8.0)).y(),0.5, GREEN);
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
    draw_circle((new_position+vec2(8.0,0.0)).x(),(new_position+vec2(8.0,0.0)).y(),0.5, GREEN);
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(8.0,8.0));
    draw_circle((new_position+vec2(8.0,8.0)).x(),(new_position+vec2(8.0,8.0)).y(),0.5, GREEN);
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
    draw_circle((new_position+vec2(0.0,16.0)).x(),(new_position+vec2(8.0,8.0)).y(),0.5, GREEN);
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position+vec2(8.0,16.0));
    draw_circle((new_position+vec2(8.0,16.0)).x(),(new_position+vec2(8.0,8.0)).y(),0.5, GREEN);
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
    let image = Image::from_file_with_format(include_bytes!("../../assets/player.png"), None);
    let spritesheet: Texture2D = load_texture_from_image(&image);
    set_texture_filter(spritesheet,FilterMode::Nearest);
    spritesheet
}