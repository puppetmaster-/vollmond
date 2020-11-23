//https://github.com/not-fl3/macroquad
// https://github.com/not-fl3/miniquad
// run www server with basic-http-server -x

mod assets;
mod utils;
mod tilemap;
mod scene;
mod entity;

use crate::scene::title::Title;
use crate::scene::game::Game;
use macroquad::prelude::*;
use crate::scene::story::Story;

const MAP_ZOOM: f32 = 6.0;
const TITLE_ZOOM: f32 = 4.0;
const SIDE_ZOOM: f32 = 4.0;
const BACKGROUND_COLOR: Color = Color([27, 25, 25, 255]);
const FONT_COLOR: Color = Color([197, 228, 243, 255]);
const DEBUG: bool = true;

#[derive(Debug)]
pub enum MainState{
    TITLE,
    STORY,
    GAME,
    EXIT,
    RUN
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut main_state = MainState::TITLE;
    let image = Image::from_file_with_format(include_bytes!("../assets/deep-night_adv.png"), None);
    let texture: Texture2D = load_texture_from_image(&image);
    set_texture_filter(texture,FilterMode::Nearest);

    let mut title = Title::init().await;
    let mut game = Game::init().await;
    let mut story = Story::init().await;

    let mut fps_buffer = vec![];
    loop {
        clear_background(BACKGROUND_COLOR);
        if DEBUG{
            show_fancy_fps(&mut fps_buffer);
        }
        match main_state {
            MainState::EXIT => break,
            MainState::TITLE => if let Some(gs) = title.run(texture) { main_state = gs },
            MainState::GAME => if let Some(gs) = game.run(texture) { main_state = gs },
            MainState::STORY => if let Some(gs) = story.run(texture) { main_state = gs },
            _ => {}
        }
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Vollmond".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[allow(dead_code)]
fn show_fancy_fps(fps_buffer: &mut Vec<f32>){

    let time = get_frame_time() * 5000.0;
    fps_buffer.insert(0, time);

    for (x, time) in fps_buffer.iter().enumerate() {
        draw_line((x+10) as f32, 100.0, (x+10) as f32, 80.0 - time/4.0, 1.0, BLUE);
    }
    draw_text(&format!("{}",time),20.0, 110.0,16.0, WHITE);

    if fps_buffer.len() as f32 > 100.0 {
        fps_buffer.resize(100.0 as usize, 0.0);
    }
}

