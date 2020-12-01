use macroquad::prelude::*;
use crate::{MAP_ZOOM, MainState, SIDE_ZOOM};
use crate::tilemap::Tilemap;
use crate::entity::player_map::PlayerMap;
use std::future::Future;
use crate::entity::player_side::{PlayerSide, SPAWN_ID};
use std::collections::HashMap;
use keyframe::Keyframe;
use keyframe::functions::{EaseInOut, Linear, EaseOut, EaseIn};
use crate::utils::tween::Tween;

const OFFSET_CAMERA: f32 = 15.0;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState{
    MAP,
    MAP_HOUSE,
    HOUSE,
    MAP_CEMETERY,
    CEMETERY,
    MAP_ICE,
    ICE,
    MAP_FOREST,
    FOREST,
    MAP_SWAMP,
    SWAMP,
    MAP_SAND,
    SAND,
    MAP_ZELDA1,
    ZELDA1,
    MAP_ZELDA2,
    ZELDA2,
    MAP_ZELDA3,
    ZELDA3,
}

pub struct Game {
    map_texture: Texture2D,
    side_texture: Texture2D,
    player_map: PlayerMap,
    player_side: PlayerSide,
    map_tilemap: Tilemap,
    tilemaps: HashMap<GameState,Tilemap>,
    current_tilemap_key: GameState,
    camera_map: Camera2D,
    camera_side: Camera2D,
    camera_sky: Camera2D,
    game_state: GameState,
    init_sidemap: bool,
    item_tween: Tween,
}

impl Game{
    pub fn init() -> impl Future<Output = Game> {
        async move {
            let tween = Tween::from_keyframes(vec![
                Keyframe::new(0.0,0.0,EaseOut),
                Keyframe::new(4.0,0.5,EaseOut),
                Keyframe::new(0.0,1.0,EaseIn)],0,2,true);
            let map_texture = get_map_texture();
            let side_texture = get_side_texture();
            let map_tilemap = get_map_tilemap();
            let player_map = PlayerMap::new(&map_tilemap);
            let mut tilemaps = HashMap::new();
            tilemaps.insert(GameState::CEMETERY,get_side_tilemap(include_bytes!("../../assets/maps/cemetery.json").to_vec()));
            tilemaps.insert(GameState::FOREST,get_side_tilemap(include_bytes!("../../assets/maps/green2.json").to_vec()));
            tilemaps.insert(GameState::ICE,get_side_tilemap(include_bytes!("../../assets/maps/ice.json").to_vec()));
            tilemaps.insert(GameState::SAND,get_side_tilemap(include_bytes!("../../assets/maps/green.json").to_vec()));
            tilemaps.insert(GameState::SWAMP,get_side_tilemap(include_bytes!("../../assets/maps/swamp.json").to_vec()));
            tilemaps.insert(GameState::ZELDA1,get_side_tilemap(include_bytes!("../../assets/maps/zelda.json").to_vec()));
            tilemaps.insert(GameState::ZELDA2,get_side_tilemap(include_bytes!("../../assets/maps/zelda.json").to_vec()));
            tilemaps.insert(GameState::ZELDA3,get_side_tilemap(include_bytes!("../../assets/maps/green.json").to_vec()));
            let player_side = PlayerSide::new();

            let camera_map = Camera2D {
                zoom: vec2(MAP_ZOOM / screen_width() * 2.0, -MAP_ZOOM / screen_height() * 2.0),
                target: player_map.position(),
                ..Default::default()
            };
            let camera_side = Camera2D {
                zoom: vec2(SIDE_ZOOM / screen_width() * 2.0, -SIDE_ZOOM / screen_height() * 2.0),
                target: player_side.position()-vec2(0.0,OFFSET_CAMERA),
                ..Default::default()
            };
            let camera_sky = Camera2D {
                zoom: vec2(SIDE_ZOOM / screen_width() * 2.0, -SIDE_ZOOM / screen_height() * 2.0),
                target: player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0),
                ..Default::default()
            };


            Game {
                map_texture,
                side_texture,
                player_map,
                player_side,
                map_tilemap,
                tilemaps,
                current_tilemap_key: GameState::MAP_CEMETERY,
                camera_map,
                camera_side,
                camera_sky,
                game_state: GameState::MAP,
                init_sidemap: true,
                item_tween: tween,
            }
        }
    }

    pub fn run(&mut self) -> Option<MainState>{
        self.item_tween.update();
        match self.game_state {
            GameState::MAP => {
                if let Some(gs) = self.player_map.update(&self.map_tilemap){
                    if gs == GameState::HOUSE {
                        if self.player_side.ingredients == 4 {
                            self.game_state = gs;
                        }
                    }else{
                        self.game_state = gs;
                    }
                }
                update_map_camera(self, self.player_map.position());
                set_camera(self.camera_map);

                self.map_tilemap.draw(self.map_texture, vec2(0.0, 0.0), None);
                self.player_map.draw(self.map_texture);

                set_default_camera();
                process_action()
            },
            GameState::MAP_HOUSE => {
                self.game_state = GameState::HOUSE;
                None
            },
            GameState::HOUSE => {
                Some(MainState::END)
            },
            GameState::MAP_CEMETERY => {
                self.current_tilemap_key = GameState::CEMETERY;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            GameState::MAP_ICE => {
                self.current_tilemap_key = GameState::ICE;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            GameState::MAP_SWAMP => {
                self.current_tilemap_key = GameState::SWAMP;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            GameState::MAP_SAND => {
                self.current_tilemap_key = GameState::SAND;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            GameState::MAP_FOREST=> {
                self.current_tilemap_key = GameState::FOREST;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            GameState::MAP_ZELDA1=> {
                self.current_tilemap_key = GameState::ZELDA1;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            GameState::MAP_ZELDA2=> {
                self.current_tilemap_key = GameState::ZELDA2;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            GameState::MAP_ZELDA3=> {
                self.current_tilemap_key = GameState::ZELDA3;
                self.player_side.position = self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),SPAWN_ID)[0];
                self.camera_side.target = self.player_side.position()-vec2(4.0,OFFSET_CAMERA);
                self.camera_sky.target = self.player_side.position()-vec2(-100.0,OFFSET_CAMERA-10.0);
                self.game_state = self.current_tilemap_key.clone();
                None
            },
            _ => {
                if let Some(gs) = self.player_side.update(self.tilemaps.get_mut(&self.current_tilemap_key).unwrap()){
                    self.game_state = gs;
                }
                update_sky_camera(self);
                set_camera(self.camera_sky);
                self.tilemaps.get(&self.current_tilemap_key).unwrap().draw(self.side_texture, vec2(0.0, 0.0), Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("sky")));
                set_default_camera();
                update_side_camera(self, self.player_side.position());
                set_camera(self.camera_side);
                self.tilemaps.get(&self.current_tilemap_key).unwrap().draw(self.side_texture, vec2(0.0, 0.0), Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("background")));
                self.tilemaps.get(&self.current_tilemap_key).unwrap().draw(self.side_texture, vec2(0.0, 0.0), Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("map")));
                //draw Items
                for id in 474..479{
                    let item_pos= self.tilemaps.get(&self.current_tilemap_key).unwrap().get_all_position_from_id(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("logic"),id);
                    if item_pos.len() > 0{
                        draw_texture_ex(self.side_texture,item_pos[0].x(),(item_pos[0].y() + self.item_tween.value()).round(),WHITE,DrawTextureParams{
                            source: Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_clip_from_id(id)),
                            ..Default::default()
                        });
                    }
                }
                self.player_side.draw();
                self.tilemaps.get(&self.current_tilemap_key).unwrap().draw(self.side_texture, vec2(0.0, 0.0), Some(self.tilemaps.get(&self.current_tilemap_key).unwrap().get_layer_id("front")));
                set_default_camera();
                None
            }
        }

    }
}

fn update_map_camera(game: &mut Game, new_target: Vec2){
    game.camera_map.target = new_target;
    game.camera_map.zoom =  vec2(MAP_ZOOM / screen_width()* 2.0, -MAP_ZOOM / screen_height()* 2.0);
}

fn update_side_camera(game: &mut Game, new_target: Vec2){
    if new_target.x() > 290.0 &&  new_target.x() < 670.0{
        game.camera_side.target.set_x(new_target.x());
    }
    game.camera_side.zoom =  vec2(SIDE_ZOOM/ screen_width()* 2.0, -SIDE_ZOOM / screen_height()* 2.0);
}
fn update_sky_camera(game: &mut Game){
    game.camera_sky.zoom =  vec2(SIDE_ZOOM/ screen_width()* 2.0, -SIDE_ZOOM / screen_height()* 2.0);
}

fn get_map_texture() -> Texture2D{
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/map.png"), None);
    let texture: Texture2D = load_texture_from_image(&image);
    set_texture_filter(texture,FilterMode::Nearest);
    texture
}
fn get_side_texture() -> Texture2D{
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/side.png"), None);
    let texture: Texture2D = load_texture_from_image(&image);
    set_texture_filter(texture,FilterMode::Nearest);
    texture
}

fn get_map_tilemap() -> Tilemap{
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

fn process_action() -> Option<MainState>{
    None
}