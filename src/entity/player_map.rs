use crate::constants::FLOAT_CMP_ERROR_MARGIN;
use crate::scene::game::GameState;
use crate::tilemap::tile_animation::TileAnimation;
use crate::tilemap::Tilemap;
use crate::utils::timer::Timer;
use crate::DEBUG;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::collections::HashMap;
use std::time::Duration;

// player
const MOVING_SPEED: f32 = 0.8;
const SPAWN_ID: u32 = 507;
const PLAYER_ID_LEFT: u32 = 571;
const PLAYER_ID_RIGHT: u32 = 570;
const PLAYER_ID_UP: u32 = 568;
const PLAYER_ID_DOWN: u32 = 569;

// side maps
const HAUS: u32 = 508;
const CEMETRY: u32 = 509;
const ICE: u32 = 518;
const SWAMP: u32 = 512;
const SAND: u32 = 517;
const ZELDA1: u32 = 513;
const ZELDA2: u32 = 514;
const ZELDA3: u32 = 515;
const FOREST: u32 = 516;
pub const SECRET: u32 = 519;

// moving speed
const GROUND_GRASS: u32 = 533;
const GROUND_ICE: u32 = 535;
const GROUND_ROCK: u32 = 534;
const GROUND_SAND: u32 = 536;
const GROUND_SWAMP: u32 = 537;
const GROUND_WATER: u32 = 524;
//const GROUND_STREET: u32 = 540;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AnimationState {
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown,
    StandRight,
    StandLeft,
    StandUp,
    StandDown,
}

pub struct PlayerMap {
    pub position: Vec2,
    collide_color: Color,
    pub last_id: Option<u32>,
    timer: Timer,
    animations: HashMap<AnimationState, TileAnimation>,
    animation_state: AnimationState,
}

impl PlayerMap {
    pub fn new(tilemap: &Tilemap) -> Self {
        let pos = tilemap.get_all_position_from_id(tilemap.get_layer_id("logic"), SPAWN_ID)[0];
        Self {
            position: pos,
            collide_color: SKYBLUE,
            last_id: None,
            timer: Timer::new_sec(1),
            animations: get_animations(),
            animation_state: AnimationState::StandDown,
        }
    }
    pub fn update(&mut self, tilemap: &Tilemap) -> Option<GameState> {
        if self.timer.finished() {
            self.animations.get_mut(&self.animation_state).unwrap().advance();

            let id_center = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position_rounded() + vec2(4.0, 4.0));
            let ground = tilemap.get_id_at_position(tilemap.get_layer_id("background"), self.position_rounded() + vec2(4.0, 4.0));

            let moving_speed_factor = match ground {
                Some(id) => match id {
                    GROUND_GRASS => 0.9,
                    GROUND_ROCK => 0.7,
                    GROUND_SAND => 0.6,
                    GROUND_ICE => 1.1,
                    GROUND_SWAMP => 0.4,
                    GROUND_WATER => 0.3,
                    _ => 1.0,
                },
                _ => 1.0,
            };

            self.collide_color = SKYBLUE;
            let velocity = MOVING_SPEED * moving_speed_factor;
            let mut new_x = self.position.x();
            let mut new_y = self.position.y();

            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                if can_walk_up(vec2(self.position.x(), self.position.y() - velocity).round(), tilemap) {
                    if self.animation_state != AnimationState::WalkUp {
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::WalkUp;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    new_y = self.position.y() - velocity;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                if can_walk_down(vec2(self.position.x(), self.position.y() + velocity).round(), tilemap) {
                    if self.animation_state != AnimationState::WalkDown {
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::WalkDown;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    new_y = self.position.y() + velocity;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                if can_walk_left(vec2(self.position.x() - velocity, self.position.y()).round(), tilemap) {
                    if self.animation_state != AnimationState::WalkLeft {
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::WalkLeft;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    new_x = self.position.x() - velocity;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                if can_walk_right(vec2(self.position.x() + velocity, self.position.y()).round(), tilemap) {
                    if self.animation_state != AnimationState::WalkRight {
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::WalkRight;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    new_x = self.position.x() + velocity;
                } else {
                    self.collide_color = GOLD;
                }
            } else {
                match self.animation_state {
                    AnimationState::WalkLeft => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.animation_state = AnimationState::StandLeft;
                        }
                    }
                    AnimationState::WalkRight => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.animation_state = AnimationState::StandRight;
                        }
                    }
                    AnimationState::WalkUp => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.animation_state = AnimationState::StandUp;
                        }
                    }
                    AnimationState::WalkDown => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.animation_state = AnimationState::StandDown;
                        }
                    }
                    _ => {}
                }
            };

            self.position.set_x(new_x);
            self.position.set_y(new_y);

            // map side level logic
            if id_center == Some(HAUS) && self.last_id != Some(HAUS) {
                self.last_id = Some(HAUS);
                Some(GameState::HOUSE)
            } else if id_center == Some(CEMETRY) && self.last_id != Some(CEMETRY) {
                self.last_id = Some(CEMETRY);
                Some(GameState::MapCemetery)
            } else if id_center == Some(ICE) && self.last_id != Some(ICE) {
                self.last_id = Some(ICE);
                Some(GameState::MapIce)
            } else if id_center == Some(SAND) && self.last_id != Some(SAND) {
                self.last_id = Some(SAND);
                Some(GameState::MapSand)
            } else if id_center == Some(SWAMP) && self.last_id != Some(SWAMP) {
                self.last_id = Some(SWAMP);
                Some(GameState::MapSwamp)
            } else if id_center == Some(FOREST) && self.last_id != Some(FOREST) {
                self.last_id = Some(FOREST);
                Some(GameState::MapForest)
            } else if id_center == Some(ZELDA1) && self.last_id != Some(ZELDA1) {
                self.last_id = Some(ZELDA1);
                Some(GameState::MapZelda1)
            } else if id_center == Some(ZELDA2) && self.last_id != Some(ZELDA2) {
                self.last_id = Some(ZELDA2);
                Some(GameState::MapZelda2)
            } else if id_center == Some(ZELDA3) && self.last_id != Some(ZELDA3) {
                self.last_id = Some(ZELDA3);
                Some(GameState::MapZelda3)
            } else if id_center == Some(SECRET) && self.last_id != Some(SECRET) {
                self.last_id = Some(SECRET);
                None
            } else if id_center.is_none() {
                self.last_id = None;
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn position_rounded(&self) -> Vec2 {
        self.position.round()
    }

    pub fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.position_rounded().x(),
            self.position_rounded().y(),
            WHITE,
            DrawTextureParams {
                source: self.animations.get(&self.animation_state).unwrap().source(),
                ..Default::default()
            },
        );
        if DEBUG {
            draw_circle(self.position_rounded().x(), self.position_rounded().y(), 0.5, RED);
            draw_rectangle_lines(self.position_rounded().x(), self.position_rounded().y(), 8.0, 8.0, 0.1, self.collide_color);
            draw_circle((self.position_rounded() + vec2(2.0, 3.0)).x(), (self.position_rounded() + vec2(2.0, 3.0)).y(), 0.5, YELLOW);
            draw_circle((self.position_rounded() + vec2(2.0, 7.0)).x(), (self.position_rounded() + vec2(2.0, 7.0)).y(), 0.5, PINK);
            draw_circle((self.position_rounded() + vec2(5.5, 7.0)).x(), (self.position_rounded() + vec2(5.5, 7.0)).y(), 0.5, GREEN);
            draw_circle((self.position_rounded() + vec2(5.5, 3.0)).x(), (self.position_rounded() + vec2(5.5, 3.0)).y(), 0.5, BLUE);
        }
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 3.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 7.0));
    !is_id_colliding(id) && !is_id_colliding(id2)
}

fn can_walk_right(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.5, 3.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.5, 7.0));
    !is_id_colliding(id) && !is_id_colliding(id2)
}

fn can_walk_up(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 3.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.5, 3.0));
    !is_id_colliding(id) && !is_id_colliding(id2)
}

fn can_walk_down(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 7.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.5, 7.0));

    !is_id_colliding(id) && !is_id_colliding(id2)
}

fn is_id_colliding(id: Option<u32>) -> bool {
    if let Some(i) = id {
        if i >= 520 && i <= 532 {
            return true;
        }
    }
    false
}

fn get_animations() -> HashMap<AnimationState, TileAnimation> {
    let player_tilemap = Tilemap::new(Rect::new(0.0, 0.0, 104.0, 352.0), 8, 8, 13, 44);
    let mut hashmap = HashMap::new();
    hashmap.insert(
        AnimationState::WalkUp,
        TileAnimation::new(&player_tilemap, &[6, 1, 7, 1], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::WalkDown,
        TileAnimation::new(&player_tilemap, &[4, 0, 5, 0], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::WalkLeft,
        TileAnimation::new(&player_tilemap, &[10, 3, 11, 3], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::WalkRight,
        TileAnimation::new(&player_tilemap, &[8, 2, 9, 2], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::StandUp,
        TileAnimation::new(&player_tilemap, &[1], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::StandDown,
        TileAnimation::new(&player_tilemap, &[0], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::StandLeft,
        TileAnimation::new(&player_tilemap, &[3], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::StandRight,
        TileAnimation::new(&player_tilemap, &[2], vec![Duration::from_millis(80)]),
    );
    hashmap
}
