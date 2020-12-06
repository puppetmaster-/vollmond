use crate::{MainState, FONT_COLOR, TITLE_ZOOM};
use macroquad::prelude::*;
use std::future::Future;

pub struct End {
    camera: Camera2D,
    font: Font,
    text1: String,
}

impl End {
    pub fn init() -> impl Future<Output = End> {
        async move {
            let camera = Camera2D {
                zoom: vec2(
                    TITLE_ZOOM / screen_width() * 2.0,
                    -TITLE_ZOOM / screen_height() * 2.0,
                ),
                target: vec2(0.0, 0.0),
                ..Default::default()
            };
            // todo write end text
            let font =
                load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/GothicPixels.ttf"));
            let text1 = "You managed to bring\nthe four ingredients in time.\nHere you have the potion.\n\nThanks for playing my game.\n\n";
            End {
                camera,
                font,
                text1: text1.to_string(),
            }
        }
    }

    pub fn run(&mut self, secrets: u8) -> Option<MainState> {
        update_camera(self, vec2(0.0, 0.0));
        set_camera(self.camera);
        //draw_texture_ex(self.texture1, 0.0, 0.0, WHITE, Default::default());
        set_default_camera();
        let tp = TextParams {
            font: self.font,
            font_size: 80,
            font_scale: 0.5,
            color: FONT_COLOR,
        };
        for (i, line) in self.text1.split("\n").enumerate() {
            draw_text_ex(
                line,
                (screen_width() / 2.0) - 350.0,
                (screen_height() / 2.0) - 350.0 + i as f32 * 80.0,
                tp,
            );
        }
        draw_text_ex(
            format!("{} / 3 secrets found", secrets).as_str(),
            (screen_width() / 2.0) - 180.0,
            (screen_height() / 2.0) + 200.0,
            tp,
        );

        if get_last_key_pressed().is_some() {
            return Some(MainState::TITLE);
        }
        None
    }
}

fn update_camera(scene: &mut End, new_target: Vec2) {
    scene.camera.target = new_target;
    scene.camera.zoom = vec2(
        TITLE_ZOOM / screen_width() * 2.0,
        -TITLE_ZOOM / screen_height() * 2.0,
    );
}
