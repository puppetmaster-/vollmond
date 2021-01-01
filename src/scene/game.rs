use crate::entity::player_map::PlayerMap;
use crate::entity::player_side::{PlayerSide, SPAWN_ID};
use crate::tilemap::Tilemap;
use crate::utils::tween::Tween;
use crate::{MainState, DARKNESS_COLOR, MAP_WATER_COLOR, MAP_ZOOM, SIDE_ZOOM};
use keyframe::functions::{EaseIn, EaseOut};
use keyframe::Keyframe;
use macroquad::prelude::*;
use quad_snd::decoder;
use quad_snd::mixer::{Sound, SoundMixer};
use std::collections::HashMap;

const SECRET_SOUND_BYTES: &[u8] = include_bytes!("../../assets/sfx/secret3.wav");
const OFFSET_CAMERA: f32 = 15.0;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    SideMap,
    MAP,
    MapHouse,
    HOUSE,
    MapCemetery,
    CEMETERY,
    MapIce,
    ICE,
    MapForest,
    FOREST,
    MapSwamp,
    SWAMP,
    MapSand,
    SAND,
    MapZelda1,
    ZELDA1,
    MapZelda2,
    ZELDA2,
    MapZelda3,
    ZELDA3,
}

pub struct Game {
    map_texture: Texture2D,
    side_texture: Texture2D,
    player_map: PlayerMap,
    pub player_side: PlayerSide,
    map_tilemap: Tilemap,
    tilemaps: HashMap<GameState, Tilemap>,
    current_tilemap_key: GameState,
    camera_map: Camera2D,
    camera_side: Camera2D,
    camera_sky: Camera2D,
    game_state: GameState,
    init_sidemap: bool,
    item_tween: Tween,
    draw_sky: bool,
    secret_sound: Sound,
    mixer: SoundMixer,
}

impl Game {
    pub async fn init() -> Game {
        let tween = Tween::from_keyframes(vec![Keyframe::new(0.0, 0.0, EaseOut), Keyframe::new(4.0, 0.5, EaseOut), Keyframe::new(0.0, 1.0, EaseIn)], 0, 2, true);
        let map_texture = get_map_texture();
        let side_texture = get_side_texture();
        let map_tilemap = get_map_tilemap();
        let player_map = PlayerMap::new(&map_tilemap);

        let player_side = PlayerSide::new();

        let camera_map = Camera2D {
            zoom: vec2(MAP_ZOOM / screen_width() * 2.0, -MAP_ZOOM / screen_height() * 2.0),
            target: player_map.position_rounded(),
            ..Default::default()
        };
        let camera_side = Camera2D {
            zoom: vec2(SIDE_ZOOM / screen_width() * 2.0, -SIDE_ZOOM / screen_height() * 2.0),
            target: player_side.position() - vec2(0.0, OFFSET_CAMERA),
            ..Default::default()
        };
        let camera_sky = Camera2D {
            zoom: vec2(SIDE_ZOOM / screen_width() * 2.0, -SIDE_ZOOM / screen_height() * 2.0),
            target: player_side.position() - vec2(-100.0, OFFSET_CAMERA - 10.0),
            ..Default::default()
        };

        Game {
            map_texture,
            side_texture,
            player_map,
            player_side,
            map_tilemap,
            tilemaps: get_tilemaps(),
            current_tilemap_key: GameState::MapCemetery,
            camera_map,
            camera_side,
            camera_sky,
            game_state: GameState::MAP,
            init_sidemap: true,
            item_tween: tween,
            draw_sky: true,
            secret_sound: decoder::read_wav(SECRET_SOUND_BYTES).unwrap(),
            mixer: SoundMixer::new(),
        }
    }

    pub fn reset(&mut self) {
        self.tilemaps = get_tilemaps();
        self.player_side = PlayerSide::new();
        self.player_map = PlayerMap::new(&self.map_tilemap);
        self.game_state = GameState::MAP;
        self.init_sidemap = true;
    }

    pub fn run(&mut self) -> Option<MainState> {
        self.item_tween.update();
        let mut main_state= None;
        match self.game_state {
            GameState::MAP => {
                if let Some(gs) = self.player_map.update(&self.map_tilemap) {
                    if gs == GameState::HOUSE {
                        if self.player_side.ingredients == 4 {
                            self.game_state = gs;
                        }
                    } else {
                        self.game_state = gs;
                    }
                }
                if let Some(id) = self.player_map.last_id {
                    if id == 519 {
                        self.mixer.play(self.secret_sound.clone());
                        let vecs: Vec<Vec2> = vec![vec2(0.0, 8.0), vec2(0.0, -8.0), vec2(8.0, 0.0), vec2(-8.0, 0.0)];
                        for v in vecs {
                            self.map_tilemap
                                .set_tileid_at(self.map_tilemap.get_layer_id("deco"), None, self.player_map.position + vec2(4.0, 4.0) + v);
                        }
                        self.map_tilemap.set_tileid_at(self.map_tilemap.get_layer_id("logic"), None, self.player_map.position + vec2(4.0, 4.0));
                    }
                }
                update_map_camera(self, self.player_map.position_rounded());
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), MAP_WATER_COLOR);
                set_camera(self.camera_map);
                self.map_tilemap.draw(self.map_texture, vec2(0.0, 0.0), None);
                self.player_map.draw(self.map_texture);
                set_default_camera();
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), DARKNESS_COLOR);
            }
            GameState::MapHouse => {
                self.game_state = GameState::HOUSE;
            }
            GameState::HOUSE => {main_state = Some(MainState::END)},
            GameState::MapCemetery => {
                self.current_tilemap_key = GameState::CEMETERY;
                self.player_side.position = self
                    .tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position() - vec2(4.0, OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position() - vec2(-100.0, OFFSET_CAMERA - 10.0);
                self.game_state = self.current_tilemap_key.clone();
                self.draw_sky = true;
            }
            GameState::MapIce => {
                self.current_tilemap_key = GameState::ICE;
                self.player_side.position = self
                    .tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position() - vec2(4.0, OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position() - vec2(-100.0, OFFSET_CAMERA - 10.0);
                self.game_state = self.current_tilemap_key.clone();
                self.draw_sky = true;
            }
            GameState::MapSwamp => {
                self.current_tilemap_key = GameState::SWAMP;
                self.player_side.position = self
                    .tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position() - vec2(4.0, OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position() - vec2(-100.0, OFFSET_CAMERA - 10.0);
                self.game_state = self.current_tilemap_key.clone();
                self.draw_sky = true;
            }
            GameState::MapForest => {
                self.current_tilemap_key = GameState::FOREST;
                self.player_side.position = self
                    .tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position() - vec2(4.0, OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position() - vec2(-100.0, OFFSET_CAMERA - 10.0);
                self.game_state = self.current_tilemap_key.clone();
                self.draw_sky = true;
            }
            GameState::MapZelda1 => {
                self.current_tilemap_key = GameState::ZELDA1;
                self.player_side.position = self
                    .tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position() - vec2(4.0, OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position() - vec2(-100.0, OFFSET_CAMERA - 1000.0);
                self.game_state = self.current_tilemap_key.clone();
                self.draw_sky = false;
            }
            GameState::MapZelda2 => {
                self.current_tilemap_key = GameState::ZELDA2;
                self.player_side.position = self
                    .tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position() - vec2(4.0, OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position() - vec2(-100.0, OFFSET_CAMERA - 10.0);
                self.game_state = self.current_tilemap_key.clone();
                self.draw_sky = false;
            }
            GameState::MapZelda3 => {
                self.current_tilemap_key = GameState::ZELDA3;
                self.player_side.position = self
                    .tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position() - vec2(4.0, OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position() - vec2(-100.0, OFFSET_CAMERA - 10.0);
                self.game_state = self.current_tilemap_key.clone();
                self.draw_sky = true;
            }
            _ => {
                if let Some(gs) = self.player_side.update(self.tilemaps.get_mut(&self.current_tilemap_key).unwrap()) {
                    self.game_state = gs;
                }
                update_sky_camera(self);
                set_camera(self.camera_sky);
                if self.draw_sky {
                    self.tilemaps
                        .get(&self.current_tilemap_key)
                        .unwrap()
                        .draw(self.side_texture, vec2(0.0, 0.0), Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("sky")));
                }
                set_default_camera();
                update_side_camera(self, self.player_side.position());
                set_camera(self.camera_side);
                self.tilemaps.get(&self.current_tilemap_key).unwrap().draw(
                    self.side_texture,
                    vec2(0.0, 0.0),
                    Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("background")),
                );
                self.tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .draw(self.side_texture, vec2(0.0, 0.0), Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("map")));
                //draw Items
                for id in 474..479 {
                    let item_pos = self
                        .tilemaps
                        .get(&self.current_tilemap_key)
                        .unwrap()
                        .get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"), id);
                    if !item_pos.is_empty() {
                        draw_texture_ex(
                            self.side_texture,
                            item_pos[0].x(),
                            (item_pos[0].y() + self.item_tween.value()).round(),
                            WHITE,
                            DrawTextureParams {
                                source: Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_clip_from_id(id)),
                                ..Default::default()
                            },
                        );
                    }
                }
                self.player_side.draw();
                self.tilemaps
                    .get(&self.current_tilemap_key)
                    .unwrap()
                    .draw(self.side_texture, vec2(0.0, 0.0), Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("front")));
                set_default_camera();
            }
        }
        self.mixer.frame();
        main_state
    }
}

fn update_map_camera(game: &mut Game, new_target: Vec2) {
    game.camera_map.target = new_target;
    game.camera_map.zoom = vec2(MAP_ZOOM / screen_width() * 2.0, -MAP_ZOOM / screen_height() * 2.0);
}

fn update_side_camera(game: &mut Game, new_target: Vec2) {
    if new_target.x() > 290.0 && new_target.x() < 670.0 {
        game.camera_side.target.set_x(new_target.x());
    }
    game.camera_side.zoom = vec2(SIDE_ZOOM / screen_width() * 2.0, -SIDE_ZOOM / screen_height() * 2.0);
}
fn update_sky_camera(game: &mut Game) {
    game.camera_sky.zoom = vec2(SIDE_ZOOM / screen_width() * 2.0, -SIDE_ZOOM / screen_height() * 2.0);
}

fn get_map_texture() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/map.png"), None);
    let texture: Texture2D = load_texture_from_image(&image);
    set_texture_filter(texture, FilterMode::Nearest);
    texture
}
fn get_side_texture() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/side.png"), None);
    let texture: Texture2D = load_texture_from_image(&image);
    set_texture_filter(texture, FilterMode::Nearest);
    texture
}

fn get_map_tilemap() -> Tilemap {
    let tiles_json_vec = include_bytes!("../../assets/maps/map.json").to_vec();
    let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0, 0.0, 104.0, 352.0), String::from_utf8(tiles_json_vec).unwrap().as_str());
    tilemap.visibility(tilemap.get_layer_id("logic"), false);
    tilemap
}

fn get_side_tilemap(json_vec: Vec<u8>) -> Tilemap {
    let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0, 0.0, 104.0, 336.0), String::from_utf8(json_vec).unwrap().as_str());
    tilemap.visibility(tilemap.get_layer_id("logic"), false);
    tilemap.visibility(tilemap.get_layer_id("collision"), false);
    tilemap
}

fn get_tilemaps() -> HashMap<GameState, Tilemap> {
    let mut tilemaps = HashMap::new();
    tilemaps.insert(GameState::CEMETERY, get_side_tilemap(include_bytes!("../../assets/maps/cemetery.json").to_vec())); //Haar
    tilemaps.insert(GameState::FOREST, get_side_tilemap(include_bytes!("../../assets/maps/green.json").to_vec())); //blume
    tilemaps.insert(GameState::ICE, get_side_tilemap(include_bytes!("../../assets/maps/ice.json").to_vec())); //Stein
    tilemaps.insert(GameState::SWAMP, get_side_tilemap(include_bytes!("../../assets/maps/swamp.json").to_vec())); // Frucht
    tilemaps.insert(GameState::ZELDA1, get_side_tilemap(include_bytes!("../../assets/maps/zelda.json").to_vec())); // Zelda
    tilemaps.insert(GameState::ZELDA2, get_side_tilemap(include_bytes!("../../assets/maps/zelda.json").to_vec())); // Zelda
    tilemaps.insert(GameState::ZELDA3, get_side_tilemap(include_bytes!("../../assets/maps/tree.json").to_vec())); // Zelda
    tilemaps
}
