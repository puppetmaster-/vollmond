use macroquad::texture::Texture2D;
use macroquad::prelude::*;
use std::future::Future;
use crate::MainState;

pub struct Title{
}

impl Title{
    pub fn init() -> impl Future<Output = Title>{
        async move {
            Title{}
        }
    }

    pub fn run(&mut self, _texture: Texture2D) -> Option<MainState>{
        process_action()
    }
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