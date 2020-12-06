use crate::scene::game::GameState;
use crate::tilemap::Tilemap;
use crate::utils::timer::Timer;
use crate::DEBUG;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

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

// moving speed
const GROUND_GRASS: u32 = 533;
const GROUND_ICE: u32 = 535;
const GROUND_ROCK: u32 = 534;
const GROUND_SAND: u32 = 536;
const GROUND_SWAMP: u32 = 537;
//const GROUND_STREET: u32 = 540;

pub struct PlayerMap {
    pub position: Vec2,
    source_left: Option<Rect>,
    source_right: Option<Rect>,
    source_up: Option<Rect>,
    source_down: Option<Rect>,
    source: Option<Rect>,
    collide_color: Color,
    last_id: Option<u32>,
    timer: Timer,
}

impl PlayerMap {
    pub fn new(tilemap: &Tilemap) -> Self {
        let pos = tilemap.get_all_position_from_id(tilemap.get_layer_id("logic"), SPAWN_ID)[0];
        Self {
            position: pos,
            source_left: Some(tilemap.get_clip_from_id(PLAYER_ID_LEFT)),
            source_right: Some(tilemap.get_clip_from_id(PLAYER_ID_RIGHT)),
            source_up: Some(tilemap.get_clip_from_id(PLAYER_ID_UP)),
            source_down: Some(tilemap.get_clip_from_id(PLAYER_ID_DOWN)),
            source: Some(tilemap.get_clip_from_id(PLAYER_ID_DOWN)),
            collide_color: SKYBLUE,
            last_id: None,
            timer: Timer::new_sec(1),
        }
    }
    pub fn update(&mut self, tilemap: &Tilemap) -> Option<GameState> {
        if self.timer.finished() {
            let id_center = tilemap.get_id_at_position(
                tilemap.get_layer_id("logic"),
                self.position() + vec2(4.0, 4.0),
            );
            let ground = tilemap.get_id_at_position(
                tilemap.get_layer_id("background"),
                self.position() + vec2(4.0, 4.0),
            );

            let moving_speed_factor = match ground {
                Some(id) => match id {
                    GROUND_GRASS => 0.9,
                    GROUND_ROCK => 0.7,
                    GROUND_SAND => 0.6,
                    GROUND_ICE => 1.1,
                    GROUND_SWAMP => 0.4,
                    _ => 1.0,
                },
                _ => 1.0,
            };

            self.collide_color = SKYBLUE;
            let velocity = MOVING_SPEED * moving_speed_factor;
            let mut new_x = self.position.x();
            let mut new_y = self.position.y();

            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                if can_walk_up(
                    vec2(self.position.x(), self.position.y() - velocity),
                    tilemap,
                ) {
                    new_y = self.position.y() - velocity;
                    self.source = self.source_up;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                if can_walk_down(
                    vec2(self.position.x(), self.position.y() + velocity),
                    tilemap,
                ) {
                    new_y = self.position.y() + velocity;
                    self.source = self.source_down;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                if can_walk_left(
                    vec2(self.position.x() - velocity, self.position.y()),
                    tilemap,
                ) {
                    new_x = self.position.x() - velocity;
                    self.source = self.source_left;
                } else {
                    self.collide_color = GOLD;
                }
            } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                if can_walk_right(
                    vec2(self.position.x() + velocity, self.position.y()),
                    tilemap,
                ) {
                    new_x = self.position.x() + velocity;
                    self.source = self.source_right;
                } else {
                    self.collide_color = GOLD;
                }
            } else {
            };

            if new_x == self.position.x() {
                if self.position.x() % 1.0 > 0.0 {
                    self.position = vec2(
                        self.position.x() - self.position.x() % 1.0,
                        self.position.y(),
                    );
                }
                self.position.set_y(new_y);
            } else if new_y == self.position.y() {
                if self.position.y() % 1.0 > 0.0 {
                    self.position = vec2(
                        self.position.x(),
                        self.position.y() - self.position.y() % 1.0,
                    );
                }
                self.position.set_x(new_x);
            } else {
                self.position.set_x(new_x);
                self.position.set_y(new_y);
            }

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

    pub fn position(&self) -> Vec2 {
        self.position.round()
    }

    pub fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.position().x(),
            self.position().y(),
            WHITE,
            DrawTextureParams {
                source: self.source,
                ..Default::default()
            },
        );
        if DEBUG {
            draw_circle(
                self.position.x().round(),
                self.position.y().round(),
                0.5,
                RED,
            );
            draw_rectangle_lines(
                self.position().x(),
                self.position().y(),
                7.0,
                7.0,
                0.1,
                self.collide_color,
            );
            draw_circle(
                (self.position() + vec2(4.0, 4.0)).x(),
                (self.position() + vec2(4.0, 4.0)).y(),
                0.5,
                RED,
            );
            draw_circle(
                (self.position + vec2(2.0, 3.0)).x(),
                (self.position + vec2(2.0, 3.0)).y(),
                0.5,
                GREEN,
            );
            draw_circle(
                (self.position + vec2(2.0, 8.0)).x(),
                (self.position + vec2(2.0, 8.0)).y(),
                0.5,
                GREEN,
            );
            draw_circle(
                (self.position + vec2(6.0, 8.0)).x(),
                (self.position + vec2(6.0, 8.0)).y(),
                0.5,
                GREEN,
            );
            draw_circle(
                (self.position + vec2(6.0, 3.0)).x(),
                (self.position + vec2(6.0, 3.0)).y(),
                0.5,
                GREEN,
            );
        }
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 3.0));
    let id2 =
        tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 7.0));
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

fn can_walk_right(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.5, 3.0));
    let id2 =
        tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.5, 7.0));
    match id {
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        }
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        }
        _ => {}
    }
    true
}

fn can_walk_up(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 3.0));
    let id2 =
        tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.0, 3.0));
    match id {
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        }
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        }
        _ => {}
    }
    true
}

fn can_walk_down(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(2.0, 7.0));
    let id2 =
        tilemap.get_id_at_position(tilemap.get_layer_id("map"), new_position + vec2(5.0, 7.0));
    match id {
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        }
        _ => {}
    }
    match id2 {
        Some(i) => {
            if i >= 520 && i <= 532 {
                return false;
            }
        }
        _ => {}
    }
    true
}
