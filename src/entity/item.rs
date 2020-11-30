use crate::tilemap::tile_animation::TileAnimation;
use macroquad::prelude::{Vec2, draw_texture_ex, WHITE, DrawTextureParams, Texture2D, draw_circle, draw_rectangle_lines, GREEN, RED, Rect};
use crate::utils::tween::Tween;
use keyframe::Keyframe;
use keyframe::functions::{EaseInOut, EaseOut};
use crate::tilemap::Tilemap;
use std::time::Duration;
use crate::DEBUG;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ItemType{
    FLOWER
}

pub struct Item{
    item_type: ItemType,
    position: Vec2,
    animation: TileAnimation,
    tween: Tween,
    active: bool,
}

impl Item{
    pub fn new(item_type: ItemType, position: Vec2, tilemap: Tilemap) -> Self {
        let animation = TileAnimation::new(&tilemap,&[12,24],vec![Duration::from_millis(80)]);
        let tween = Tween::from_keyframes(vec![
            Keyframe::new(0.0,0.0,EaseOut),
            Keyframe::new(8.0,0.5,EaseOut),
            Keyframe::new(0.0,1.0,EaseInOut)], 0,3,true);
        Self{
            item_type,
            position,
            animation,
            tween,
            active: true
        }
    }

    pub fn update(&mut self, position_to_check: Vec2) -> Option<ItemType>{
        if Rect::new(self.position.x(), self.position.y(), 8.0, 8.0).contains(position_to_check){
            Some(self.item_type.clone())
        }else{
            None
        }
    }

    pub fn position(&self) -> Vec2{
        self.position.round()
    }

    pub fn draw(&self, texture: Texture2D){
        draw_texture_ex(texture, self.position().x(), self.position().y()+self.tween.value(), WHITE,DrawTextureParams {
            source: self.animation.source(),
            ..Default::default()
        });
        if DEBUG{
            draw_circle(self.position().x(), self.position().y(),0.5, RED);
            draw_rectangle_lines(self.position().x(),self.position().y(),8.0,8.0,0.1,GREEN);
        }
    }
}