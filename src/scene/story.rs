use macroquad::texture::Texture2D;
use macroquad::prelude::*;
use std::future::Future;
use crate::{MainState, TITLE_ZOOM, FONT_COLOR};
use crate::utils::tween::Tween;
use keyframe::Keyframe;
use keyframe::functions::{EaseInOut, Linear, EaseOut, EaseIn};
use crate::scene::title::Title;

pub struct Story{
    texture1: Texture2D,
    camera: Camera2D,
}

impl Story{
    pub fn init() -> impl Future<Output = Story>{
        async move {
            let camera = Camera2D {
                zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM/ screen_height() * 2.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            };
            let image = Image::from_file_with_format(include_bytes!("../../assets/story1.png"), None);
            let texture1: Texture2D = load_texture_from_image(&image);
            set_texture_filter(texture1,FilterMode::Nearest);
            Story{
                texture1,
                camera,
            }
        }
    }

    pub fn run(&mut self, _texture: Texture2D) -> Option<MainState>{
        update_camera(self, vec2(0.0,0.0));
        set_camera(self.camera);
        draw_texture_ex(self.texture1, 0.0, 0.0, WHITE, Default::default());
        set_default_camera();
        process_action()
    }
}

fn update_camera(game: &mut Story, new_target: Vec2){
    game.camera.target = new_target;
    game.camera.zoom =  vec2(TITLE_ZOOM / screen_width()* 2.0, -TITLE_ZOOM / screen_height()* 2.0);
}

fn process_action() -> Option<MainState>{
    if get_last_key_pressed().is_some() {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            return Some(MainState::EXIT);
        } else {
            return Some(MainState::GAME);
        }
    }
    None
}

