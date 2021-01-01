use crate::{MainState, FONT_COLOR, TITLE_ZOOM};
use macroquad::prelude::*;
use quad_snd::decoder;
use quad_snd::mixer::{SoundMixer, Volume};

const MUSIC_BYTES: &[u8] = include_bytes!("../../assets/music/end.ogg");

pub struct End {
    camera: Camera2D,
    font: Font,
    text1: Vec<String>,
    start: bool,
}

impl End {
    pub async fn init() -> End {
        let camera = Camera2D {
            zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };
        // todo write end text
        let font = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/GothicPixels.ttf"));
        let t1 = "You managed to bring\nthe four ingredients in time.\nHere you have the potion.\n\nThanks for playing my game.\n\n";
        let text1 = t1.to_string().split('\n').map(String::from).collect();
        End { 
            camera, 
            font, 
            text1, 
            start: true,
        }
    }

    pub fn run(&mut self, secrets: u8,mixer: &mut SoundMixer) -> Option<MainState> {
        if self.start {
            let id = mixer.play(decoder::read_ogg(MUSIC_BYTES).unwrap());
            mixer.set_volume(id, Volume(0.6));
            self.start = false;
        }
        update_camera(self, vec2(0.0, 0.0));
        set_camera(self.camera);
        set_default_camera();
        let tp = TextParams {
            font: self.font,
            font_size: 80,
            font_scale: 0.5,
            color: FONT_COLOR,
        };
        for (i, line) in self.text1.iter().enumerate() {
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
    scene.camera.zoom = vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0);
}
