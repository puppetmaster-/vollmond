use macroquad::prelude::*;
use crate::{MAP_ZOOM, MainState, SIDE_ZOOM};
use crate::tilemap::Tilemap;
use crate::entity::player_map::PlayerMap;
use std::future::Future;
use crate::entity::player_side::PlayerSide;

const OFFSET_CAMERA: f32 = 15.0;

#[allow(dead_code)]
#[derive(Debug)]
pub enum GameState{
    HOUSE1,
    MAP,
    CEMETERY,
}

pub struct Game {
    player_map: PlayerMap,
    player_side: PlayerSide,
    map_tilemap: Tilemap,
    cemetery_tilemap: Tilemap,
    camera_map: Camera2D,
    camera_side: Camera2D,
    camera_sky: Camera2D,
    game_state: GameState
}

impl Game{
    pub fn init() -> impl Future<Output = Game> {
        async move {

            let map_tilemap = get_map_tilemap();
            let player_map = PlayerMap::new(&map_tilemap);
            let cemetery_tilemap = get_cemetery();
            let player_side = PlayerSide::new(&cemetery_tilemap); //TODO falsch

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
                player_map,
                player_side,
                map_tilemap,
                cemetery_tilemap,
                camera_map,
                camera_side,
                camera_sky,
                game_state: GameState::MAP
            }
        }
    }

    pub fn run(&mut self, texture: Texture2D) -> Option<MainState>{
        match self.game_state{
            GameState::MAP => {
                if let Some(gs) = self.player_map.update(&self.map_tilemap){
                    self.game_state = gs;
                }
                // draw
                update_map_camera(self, self.player_map.position());
                set_camera(self.camera_map);

                self.map_tilemap.draw(texture, vec2(0.0, 0.0), None);
                self.player_map.draw(texture);

                set_default_camera();
                process_action()
            },
            GameState::HOUSE1 => {
                self.game_state = GameState::MAP;
                None
            },
            GameState::CEMETERY => {
                if let Some(gs) = self.player_side.update(&self.cemetery_tilemap){
                    self.game_state = gs;
                }
                update_sky_camera(self);
                set_camera(self.camera_sky);
                self.cemetery_tilemap.draw(texture, vec2(0.0, 0.0), Some(self.cemetery_tilemap.get_layer_id("sky")));
                set_default_camera();
                update_side_camera(self, self.player_side.position());
                set_camera(self.camera_side);
                self.cemetery_tilemap.draw(texture, vec2(0.0, 0.0), Some(self.cemetery_tilemap.get_layer_id("background")));
                self.cemetery_tilemap.draw(texture, vec2(0.0, 0.0), Some(self.cemetery_tilemap.get_layer_id("map")));
                self.player_side.draw(texture);
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

fn get_map_tilemap() -> Tilemap{
    let tiles_json_vec = include_bytes!("../../assets/deep-night_adv.json").to_vec();
    let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0, 0.0, 104.0, 336.0), String::from_utf8(tiles_json_vec).unwrap().as_str());
    tilemap.visibility(tilemap.get_layer_id("logic"), false);
    tilemap
}

fn get_cemetery() -> Tilemap{
    let tiles_json_vec = include_bytes!("../../assets/side1.json").to_vec();
    let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0, 0.0, 104.0, 336.0), String::from_utf8(tiles_json_vec).unwrap().as_str());
    tilemap.visibility(tilemap.get_layer_id("logic"), false);
    tilemap.visibility(tilemap.get_layer_id("collision"), false);
    tilemap
}

fn process_action() -> Option<MainState>{
    if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
        return Some(MainState::TITLE);
    }
    None
}