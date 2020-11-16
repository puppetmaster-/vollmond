use std::collections::HashMap;
use nanoserde::{DeJson};
use macroquad::prelude::*;

#[allow(dead_code)]
impl PyxelTilemap {
    pub fn new(data: &str) -> PyxelTilemap{
        let mut pyxeltilemap: PyxelTilemap = DeJson::deserialize_json(data).unwrap();
        remodel(&mut pyxeltilemap);
        pyxeltilemap
    }
    pub fn get_id_at_position(&self, layer: Layers, position: Vec2) -> Option<i32>{
        let x = position.x() as i32/ self.tile_width;
        let y = position.y() as i32 / self.tile_height;
        let i = (x*y) as usize;
        match layer.tiles.get(i){
            Some(tile) => {
                let id = tile.id as i32;
                std::thread::spawn(move || drop(layer));
                Some(id)
            },
            _ => None
        }
    }
}

#[derive(Clone, Debug, Default, DeJson)]
#[nserde(rename = "RootInterface")]
pub struct PyxelTilemap {
    pub tileshigh: i64,
    pub tileswide: i64,
    #[nserde(rename = "tileheight")]
    pub tile_height: i32,
    #[nserde(rename = "tilewidth")]
    pub tile_width: i32,
    pub layers: Vec<Layers>,
}

#[derive(Clone, Debug, Default, DeJson)]
pub struct Layers {
    pub number: i64,
    pub tiles: Vec<Tile>,
    pub name: String,
}

#[derive(Clone, Debug, Default, DeJson)]
#[nserde(rename = "Tiles")]
pub struct Tile {
    #[nserde(rename = "tile")]
    pub id: i32,
    pub x: i32,
    pub y: i32,
    #[nserde(rename = "flipX")]
    flip_x: bool,
    index: i64,
    #[nserde(rename = "rot")]
    rotation_id: i8,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub rotation: Option<f32>,
    pub dest_size: Option<(f32,f32)>,
}

#[allow(clippy::approx_constant)]
fn pyxel_rotation(rotation: i8) ->f32{
    let rot = rotation;
    let mut return_value = 0.0;
    if rot == 1{
        return_value = 1.57;
    } else if rot == 2{
        return_value = 3.14;
    } else if rot == 3 {
        return_value = 4.71;
    }
    return_value
}

fn remodel(tilemap: &mut PyxelTilemap){
    for (_i,layer) in tilemap.layers.iter_mut().enumerate() {
        layer.tiles.retain(|t| t.id != -1);
        for tile in layer.tiles.iter_mut(){
            let mut dest_size = (tilemap.tile_width as f32,tilemap.tile_height as f32);
            let mut shift_x = 0;
            let mut shift_y = 0;

            if tile.flip_x {
                dest_size = (-tilemap.tile_width as f32,tilemap.tile_height as f32);
                if tile.rotation_id == 0 {
                    shift_x = tilemap.tile_width;
                }else if tile.rotation_id == 1 || tile.rotation_id == 2 || tile.rotation_id == 3{
                    shift_x = tilemap.tile_width;
                    shift_y = 0;
                }
            } else if tile.rotation_id == 1{
                shift_x = 0;
            } else if tile.rotation_id == 2{
                shift_x = 0;
                shift_y = 0;
            } else if tile.rotation_id == 3{
                shift_y = 0;
            }
            tile.position_x = Some((tile.x * tilemap.tile_width + shift_x) as f32);
            tile.position_y = Some((tile.y * tilemap.tile_height + shift_y) as f32);
            tile.rotation = Some(pyxel_rotation(tile.rotation_id));
            tile.dest_size = Some(dest_size);
        }
    }
}

#[allow(dead_code)]
pub fn get_tile_rectangles(texture_height: i32, texture_width: i32, tile_width: i32, tile_height: i32) ->HashMap<i32, Rect>{
    let mut id = 0;
    let mut tile_rectangles: HashMap<i32, Rect> = HashMap::new();
    let x = texture_width / tile_width;
    let y = texture_height / tile_height;
    for i in 0..x{
        for j in 0..y{
            let rec = Rect::new((j*tile_width) as f32,(i*tile_height) as f32, tile_width as f32, tile_height as f32); //switch x and y axis
            tile_rectangles.insert(id,rec);
            id +=1;
        }
    }
    tile_rectangles
}