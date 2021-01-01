use crate::constants::FLOAT_CMP_ERROR_MARGIN;
use crate::scene::game::GameState;
use crate::tilemap::tile_animation::TileAnimation;
use crate::tilemap::Tilemap;
use crate::utils::timer::Timer;
use crate::DEBUG;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use quad_snd::decoder;
use quad_snd::mixer::{Sound, SoundMixer};
use std::collections::HashMap;
use std::time::Duration;

pub const SPAWN_ID: u32 = 507;
const EXIT: u32 = 510;
//const ITEM_STONE: u32 = 474;
//const ITEM_HAIR: u32 = 476;
//const ITEM_FRUIT: u32 = 477;
//const ITEM_FLOWER: u32 = 475;
const ITEM_ZELDA: u32 = 478;

const JUMP_UP_FACTOR: f32 = 2.5;
const JUMP_DOWN_FACTOR: f32 = 2.0;
const JUMP_UP_CURVE: [f32; 10] = [8.0, 15.0, 13.0, 8.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
const JUMP_DOWN_CURVE: [f32; 5] = [1.0, 1.0, 2.0, 3.0, 5.0];

const MOVE_FACTOR: f32 = 4.0;
const MOVE_SPEED_CURVE: [f32; 8] = [1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0];
const BREAK_SPEED_CURVE: [f32; 8] = [21.0, 13.0, 8.0, 5.0, 3.0, 2.0, 1.0, 1.0];

const PICKUP_SOUND_BYTES: &[u8] = include_bytes!("../../assets/sfx/pickup.wav");
const JUMP_SOUND_BYTES: &[u8] = include_bytes!("../../assets/sfx/jump.wav");

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State {
    FLOOR,
    SLIDE,
    IDLE,
    RUN,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum JumpState {
    NOT,
    JUMP,
    AIR,
    DOWN,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AnimationState {
    RUNLEFT,
    RUNRIGHT,
    STANDLEFT,
    STANDRIGHT,
}

pub struct PlayerSide {
    pub ingredients: u8,
    pub bonus: u8,
    moving_timer: usize,
    break_timer: usize,
    air_timer: usize,
    jump_up_timer: usize,
    jump_down_timer: usize,
    direction: Vec2,
    pub position: Vec2,
    collide_color: Color,
    spritesheet: Texture2D,
    need_reset: bool,
    jump_timer: u32,
    state: State,
    jump_state: JumpState,
    animation_state: AnimationState,
    animations: HashMap<AnimationState, TileAnimation>,
    last_item_id: Option<u32>,
    timer: Timer,
    pickup_sound: Sound,
    jump_sound: Sound,
    mixer: SoundMixer,
}

impl PlayerSide {
    pub fn new() -> Self {
        let spritesheet = get_player_spritesheet();
        let animations = get_animations();
        Self {
            ingredients: 0,
            bonus: 0,
            moving_timer: 0,
            break_timer: 0,
            air_timer: 0,
            jump_up_timer: 0,
            jump_down_timer: 0,
            direction: Vec2::zero(),
            position: Vec2::zero(),
            collide_color: SKYBLUE,
            spritesheet,
            need_reset: true,
            jump_timer: 0,
            state: State::FLOOR,
            jump_state: JumpState::NOT,
            animation_state: AnimationState::STANDRIGHT,
            animations,
            last_item_id: None,
            timer: Timer::new_sec(1),
            pickup_sound: decoder::read_wav(PICKUP_SOUND_BYTES).unwrap(),
            jump_sound: decoder::read_wav(JUMP_SOUND_BYTES).unwrap(),
            mixer: SoundMixer::new(),
        }
    }
    pub fn update(&mut self, tilemap: &mut Tilemap) -> Option<GameState> {
        let mut gamestate= None; 

        // TODO can be called from game?
        if self.need_reset {
            self.state = State::IDLE;
            self.jump_timer = 0;
            self.moving_timer = 0;
            self.break_timer = BREAK_SPEED_CURVE.len();
            self.need_reset = false;
            self.last_item_id = None;
            self.timer.restart();
            self.animation_state = AnimationState::STANDRIGHT;
            for (_, a) in self.animations.iter_mut() {
                a.reset();
            }
        }

        self.animations.get_mut(&self.animation_state).unwrap().advance();

        let id_center = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position() + vec2(4.0, 4.0));

        self.collide_color = SKYBLUE;

        let delta = get_frame_time();
        let mut new_x = self.position.x();
        let mut new_y = self.position.y();

        if self.timer.finished() {
            //wait before moving
            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                let distance = 4.0 * MOVE_SPEED_CURVE[self.moving_timer] * delta;
                if can_walk_left(vec2(self.position.x() - distance, self.position.y()), tilemap) {
                    if self.animation_state != AnimationState::RUNLEFT {
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::RUNLEFT;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    self.state = State::RUN;
                    self.direction = vec2(-1.0, 0.0);
                    new_x = self.position.x() - distance;
                    if self.moving_timer < MOVE_SPEED_CURVE.len() - 1 {
                        self.moving_timer += 1;
                    }
                } else {
                    self.direction = vec2(1.0, 0.0);
                    self.break_timer = BREAK_SPEED_CURVE.len() - 3;
                    self.collide_color = PINK;
                }
            } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                let distance = MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta;
                if can_walk_right(vec2(self.position.x() + distance, self.position.y()), tilemap) {
                    if self.animation_state != AnimationState::RUNRIGHT {
                        self.animations.get_mut(&self.animation_state).unwrap().reset();
                        self.animation_state = AnimationState::RUNRIGHT;
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
                    }
                    self.state = State::RUN;
                    self.direction = vec2(1.0, 0.0);
                    new_x = self.position.x() + distance;
                    if self.moving_timer < MOVE_SPEED_CURVE.len() - 1 {
                        self.moving_timer += 1;
                    }
                } else {
                    self.direction = vec2(-1.0, 0.0);
                    self.break_timer = BREAK_SPEED_CURVE.len() - 3;
                    self.collide_color = GOLD;
                }
            } else {
                self.moving_timer = 0;
                if self.jump_state == JumpState::NOT {
                    if self.state == State::RUN {
                        self.break_timer = 0;
                        self.state = State::SLIDE
                    }
                    if self.break_timer < BREAK_SPEED_CURVE.len() - 1 {
                        let distance = (MOVE_FACTOR + 2.0) * BREAK_SPEED_CURVE[self.break_timer] * delta;
                        if self.direction.x() > 0.0 {
                            // right
                            if can_walk_right(vec2(self.position.x() + distance, self.position.y()), tilemap) {
                                new_x = self.position.x() + distance;
                            }
                        } else if can_walk_left(vec2(self.position.x() - distance, self.position.y()), tilemap) {
                            new_x = self.position.x() - distance;
                        }
                        self.break_timer += 1;
                    } else {
                        self.state = State::IDLE;
                        self.direction = vec2(0.0, 0.0);
                    }
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
            if (is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)) && (self.jump_state == JumpState::JUMP || self.jump_state == JumpState::NOT) {
                if self.jump_up_timer < JUMP_UP_CURVE.len() - 1 && can_jump_up(vec2(self.position.x(), self.position.y()), tilemap) {
                    if self.jump_state == JumpState::NOT {
                        self.mixer.play(self.jump_sound.clone());
                        self.jump_state = JumpState::JUMP;
                    }
                    self.jump_up_timer += 1;
                    //todo check if player can jump up
                    new_y = self.position.y() - JUMP_UP_FACTOR + JUMP_UP_CURVE[self.jump_up_timer] * delta;
                    new_x += self.direction.x() * 0.2;
                } else {
                    self.jump_state = JumpState::AIR;
                }
            }

            //stop jumping
            if (!is_key_down(KeyCode::Space) && !is_key_down(KeyCode::Up)) && self.jump_state == JumpState::JUMP {
                self.jump_state = JumpState::AIR;
                self.jump_up_timer = 0;
            }

            if self.jump_state == JumpState::AIR {
                if self.air_timer > JUMP_UP_CURVE.len() - 1 {
                    self.air_timer = 0;
                    self.jump_state = JumpState::DOWN;
                } else {
                    self.air_timer += 1;
                }
            }

            if self.jump_state == JumpState::DOWN || self.jump_state == JumpState::NOT {
                if can_walk_down(vec2(self.position.x(), self.position.y()), tilemap) {
                    if self.jump_down_timer < JUMP_DOWN_CURVE.len() - 1 {
                        self.jump_down_timer += 1;
                    }
                    new_y = self.position.y() + JUMP_DOWN_FACTOR + JUMP_DOWN_CURVE[self.jump_down_timer] * delta;
                    new_x += self.direction.x() * 0.2;
                    self.jump_state = JumpState::DOWN;
                } else {
                    self.jump_down_timer = 0;
                    self.jump_up_timer = 0;
                    self.jump_state = JumpState::NOT;
                    self.state = State::FLOOR;
                }
            }

            // fix for player inside wall //todo fixme
            if self.position.abs_diff_eq(vec2(new_x, new_y), FLOAT_CMP_ERROR_MARGIN) {
                if self.position.y() % 8.0 > 0.0 {
                    self.position = vec2(self.position.x(), self.position.y() - self.position.y() % 8.0);
                }
                self.direction = vec2(0.0, 0.0);
                self.position.set_x(new_x);
            } else {
                self.position.set_x(new_x);
                self.position.set_y(new_y);
            }

            // item pickup logic
            match id_center {
                Some(id) => match id {
                    SPAWN_ID => {},
                    EXIT => {
                        self.need_reset = true;
                        gamestate = Some(GameState::MAP);
                    }
                    ITEM_ZELDA => {
                        if self.last_item_id != Some(id) {
                            self.last_item_id = Some(id);
                            self.bonus += 1;
                            tilemap.replace_all_tileid(tilemap.get_layer_id("logic"), ITEM_ZELDA, None);
                            self.mixer.play(self.pickup_sound.clone());
                        }
                    }
                    _ => {
                        if self.last_item_id != Some(id) {
                            self.last_item_id = Some(id);
                            self.ingredients += 1;
                            tilemap.replace_all_tileid(tilemap.get_layer_id("logic"), id, None);
                            self.mixer.play(self.pickup_sound.clone());
                        }
                    }
                },
                _ => {},
            }
        }
        self.mixer.frame();
        gamestate
    }

    pub fn position(&self) -> Vec2 {
        self.position.round()
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.spritesheet,
            self.position().x() - 2.0,
            self.position().y(),
            WHITE,
            DrawTextureParams {
                source: self.animations.get(&self.animation_state).unwrap().source(),
                ..Default::default()
            },
        );
        if DEBUG {
            draw_circle(self.position.x().round(), self.position.y().round(), 0.5, RED);
            draw_rectangle_lines(self.position().x(), self.position().y(), 16.0, 16.0, 0.1, self.collide_color);
            draw_text(&format!("{:?}", &self.state), 400.0, 5.0, 14.0, WHITE);
            draw_circle((self.position + vec2(0.0, 1.0)).x(), (self.position + vec2(0.0, 1.0)).y(), 0.5, BLUE); //left up
            draw_circle((self.position + vec2(0.0, 15.0)).x(), (self.position + vec2(0.0, 15.0)).y(), 0.5, BLUE); //left down)
            draw_circle((self.position + vec2(7.0, 0.0)).x(), (self.position + vec2(7.0, 0.0)).y(), 0.5, LIME); //
            draw_circle((self.position + vec2(7.0, 8.0)).x(), (self.position + vec2(8.0, 8.0)).y(), 0.5, LIME);
            draw_circle((self.position + vec2(0.0, 16.0)).x(), (self.position + vec2(0.0, 16.0)).y(), 0.5, GREEN);
            draw_circle((self.position + vec2(8.0, 16.0)).x(), (self.position + vec2(8.0, 16.0)).y(), 0.5, GREEN);
        }
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(0.0, 1.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(0.0, 15.0));
    if let Some(i) = id {
        if i == 520 {
            return false;
        }
    }
    if let Some(i) = id2 {
        if i == 520 {
            return false;
        }
    }
    true
}

fn can_walk_right(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(8.0, 0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(8.0, 8.0));
    if let Some(i) = id {
        if i == 520 {
            return false;
        }
    }

    if let Some(i) = id2 {
        if i == 520 {
            return false;
        }
    }
    true
}

fn can_jump_up(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(0.0, 0.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(8.0, 0.0));
    if let Some(i) = id {
        if i == 520 {
            return false;
        }
    }
    if let Some(i) = id2 {
        if i == 520 {
            return false;
        }
    }
    true
}

fn can_walk_down(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(0.0, 16.0));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + vec2(8.0, 16.0));
    if let Some(i) = id {
        if i == 520 {
            return false;
        }
    }

    if let Some(i) = id2 {
        if i == 520 {
            return false;
        }
    }
    true
}

fn get_animations() -> HashMap<AnimationState, TileAnimation> {
    let player_tilemap = Tilemap::new(Rect::new(0.0, 0.0, 160.0, 128.0), 16, 16, 10, 8);
    let mut hashmap = HashMap::new();
    hashmap.insert(
        AnimationState::RUNRIGHT,
        TileAnimation::new(&player_tilemap, &[0, 1, 2, 3, 4, 5, 6, 7], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(
        AnimationState::RUNLEFT,
        TileAnimation::new(&player_tilemap, &[10, 11, 12, 13, 14, 15, 16, 17], vec![Duration::from_millis(80)]),
    );
    hashmap.insert(AnimationState::STANDRIGHT, TileAnimation::new(&player_tilemap, &[0, 20], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimationState::STANDLEFT, TileAnimation::new(&player_tilemap, &[10, 30], vec![Duration::from_millis(500)]));
    hashmap
}

fn get_player_spritesheet() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/player.png"), None);
    let spritesheet: Texture2D = load_texture_from_image(&image);
    set_texture_filter(spritesheet, FilterMode::Nearest);
    spritesheet
}
