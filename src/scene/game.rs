use macroquad::prelude::*;
use crate::{MAP_ZOOM, MainState};
use crate::tilemap::Tilemap;
use crate::entity::player::Player;
use std::future::Future;

#[derive(Debug)]
pub enum GameState{
    HOUSE1,
    MAP,
    CEMETERY,
}

pub struct Game {
    player: Player,
    tilemap: Tilemap,
    camera: Camera2D,
    game_state: GameState
}

impl Game{
    pub fn init() -> impl Future<Output = Game> {
        async move {

            let camera = Camera2D {
                zoom: vec2(MAP_ZOOM / screen_width() * 2.0, -MAP_ZOOM / screen_height() * 2.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            };
            //let tiles_json_vec = load_file("assets/deep-night_adv.json").await.ok().unwrap();
            let tiles_json_vec = include_bytes!("../../assets/deep-night_adv.json").to_vec();
            let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0, 0.0, 104.0, 336.0), String::from_utf8(tiles_json_vec).unwrap().as_str()).await;
            tilemap.visibility(tilemap.get_layer_id("logic"), false);
            let player = Player::new(&tilemap);

            Game {
                player,
                tilemap,
                camera,
                game_state: GameState::MAP
            }
        }
    }

    pub fn run(&mut self, texture: Texture2D) -> Option<MainState>{
        match self.game_state{
            GameState::MAP => {
                self.player.update_map(&self.tilemap);
                // draw
                update_map_camera(self, self.player.position());
                set_camera(self.camera);

                self.tilemap.draw(texture,vec2(0.0,0.0),None);
                self.player.draw_map(texture);

                set_default_camera();
                process_action()
            }
            _ => None
        }

    }
}

fn update_map_camera(game: &mut Game, new_target: Vec2){
    game.camera.target = new_target;
    game.camera.zoom =  vec2(MAP_ZOOM / screen_width()* 2.0, -MAP_ZOOM / screen_height()* 2.0);
}

fn process_action() -> Option<MainState>{
    if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
        return Some(MainState::TITLE);
    }
    None
}