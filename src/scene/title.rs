use macroquad::texture::Texture2D;
use macroquad::prelude::*;
use std::future::Future;
use crate::{MainState, TITLE_ZOOM, FONT_COLOR};
use crate::utils::tween::Tween;
use keyframe::Keyframe;
use keyframe::functions::{EaseInOut, Linear, EaseOut, EaseIn};

pub struct Title{
    background: Texture2D,
    title: Texture2D,
    font: Font,
    camera: Camera2D,
    animations: Vec<Tween>,
}

impl Title{
    pub fn init() -> impl Future<Output = Title>{
        async move {
            let tween1 = Tween::from_keyframes(vec![
                Keyframe::new(0.0,0.0,EaseOut),
                Keyframe::new(8.0,0.5,EaseOut),
                Keyframe::new(0.0,1.0,EaseInOut)], 0,3,true);
            let tween2 = Tween::from_keyframes(vec![
                Keyframe::new(0.0,0.0,EaseIn),
                Keyframe::new(4.0,0.5,EaseIn),
                Keyframe::new(0.0,1.0,EaseIn)],0,2,true);
            let tween3 = Tween::from_keyframes(vec![
                Keyframe::new(0.0,0.0,Linear),
                Keyframe::new(6.283_185_5,1.0,Linear)],0,10,true);
            let tween = vec![tween1, tween2, tween3];

            let camera = Camera2D {
                zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM/ screen_height() * 2.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            };
            let image = Image::from_file_with_format(include_bytes!("../../assets/title.png"), Some(image::PNG));
            let background: Texture2D = load_texture_from_image(&image);
            set_texture_filter(background,FilterMode::Nearest);
            let image2 = Image::from_file_with_format(include_bytes!("../../assets/vollmond.png"), Some(image::PNG));
            let title: Texture2D = load_texture_from_image(&image2);
            set_texture_filter(title,FilterMode::Nearest);
            let font = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/GothicPixels.ttf"));
            Title{
                background,
                font,
                camera,
                title,
                animations: tween,
            }
        }
    }

    pub fn run(&mut self, _texture: Texture2D) -> Option<MainState>{
        self.animations[0].update();
        self.animations[1].update();
        self.animations[2].update();
        update_camera(self, vec2(0.0,0.0));
        set_camera(self.camera);
        draw_texture_ex(self.background, -100.0, -50.0, WHITE, Default::default());
        draw_texture_ex(self.title, -78.0, -90.0+self.animations[0].value(), WHITE, Default::default());
        set_default_camera();
        draw_text_ex("press any key",(screen_width()/2.0)-180.0,(screen_height()/2.0)+250.0+self.animations[1].value(),TextParams{
            font: self.font,
            font_size: 100,
            font_scale: 0.5,
            color: FONT_COLOR
        });
        process_action()
    }
}

fn update_camera(game: &mut Title, new_target: Vec2){
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

