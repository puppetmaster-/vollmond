mod pyxeledit;
pub(crate) mod tile_animation;

use crate::tilemap::pyxeledit::PyxelTilemap;
use crate::utils::vecgrid::VecGrid;
use crate::DEBUG;
use macroquad::prelude::*;
use std::collections::HashMap;

#[allow(dead_code)]
impl Tilemap {
    pub fn new(clip: Rect, tile_width: i32, tile_height: i32, width: usize, height: usize) -> Tilemap {
        Tilemap {
            width,
            height,
            viewport: DEFAULT_RECTANGLE,
            tile_height,
            tile_width,
            layers: vec![Layer {
                tiles: VecGrid::new(width, height),
                ..Layer::default()
            }],
            tile_rectangles: get_tile_rectangles(clip, tile_width, tile_height),
            layer_to_draw: DEFAULT_LAYER_TO_DRAW,
        }
    }

    pub fn from_pyxeledit(clip: Rect, data: &str) -> Tilemap {
        let pyxeltilemap = PyxelTilemap::new(data);
        transform_pyxeltilemap(clip, pyxeltilemap)
    }

    pub fn color(&mut self, color: Color) -> &Tilemap {
        if self.layer_to_draw == -1 {
            for mut l in self.layers.iter_mut() {
                l.color = color;
            }
            self
        } else {
            match self.layers.get_mut(self.layer_to_draw as usize) {
                //layer vec id not = layer.number
                None => self,
                Some(mut l) => {
                    l.color = color;
                    self
                }
            }
        }
    }

    /// just a map with tile ids
    /// neither rotation nor flipping
    pub fn set_tiles_from_map(&mut self, layer: usize, list: &[Vec<u32>]) {
        let tiles = self.create_tiles_from_map(list);
        match self.layers.get_mut(layer) {
            None => self.add_layer(tiles),
            Some(layer) => layer.tiles = tiles,
        }
    }

    pub fn viewport(&mut self, rectangle: Rect) -> &Tilemap {
        self.viewport = rectangle;
        self
    }

    pub fn get_all_position_from_id(&self, layer: usize, id: u32) -> Vec<Vec2> {
        let mut positions = Vec::new();
        if let Some(layer) = self.layers.get(layer) {
            let tiles = &*layer.tiles.get_data();
            for (i, t) in tiles.iter().enumerate() {
                if t.is_some() && t.as_ref().unwrap().id == id {
                    let x = (i % self.width) * self.tile_width as usize;
                    let y = (i / self.width) * self.tile_height as usize;
                    positions.push(Vec2::new(x as f32, y as f32));
                }
            }
        };
        positions
    }

    pub fn replace_all_tileid(&mut self, layer: usize, old_id: u32, new_id: Option<u32>) {
        if let Some(layer) = self.layers.get_mut(layer) {
            for x in 0..self.width {
                for y in 0..self.height {
                    if let Some(tile) = layer.tiles.get_mut(x, y) {
                        if tile.id == old_id {
                            if let Some(id) = new_id {
                                tile.id = id;
                            } else {
                                layer.tiles.delete(x, y);
                            }
                        }
                    }
                }
            }
        } else {
            //error!("layer{} not found!", layer);
        }
    }

    pub fn set_tileid_at(&mut self, layer: usize, new_id: u32, position: Vec2) {
        let mut pos_x = position.x() as i32;
        let mut pos_y = position.y() as i32;
        if pos_x % 8 != 0 {
            pos_x -= pos_x % 8;
        }
        if pos_y % 8 != 0 {
            pos_y -= pos_y % 8;
        }
        let x = pos_x as i32 / self.tile_width;
        let y = pos_y as i32 / self.tile_height;
        if let Some(layer) = self.layers.get_mut(layer) {
            match layer.tiles.get_mut(x as _, y as _) {
                None => layer.tiles.set(
                    Tile {
                        id: new_id,
                        x: x as i32,
                        y: y as i32,
                        position_x: (x as i32 * self.tile_width) as f32,
                        position_y: (y as i32 * self.tile_height) as f32,
                        ..Tile::default()
                    },
                    x as _,
                    y as _,
                ),
                Some(tile) => tile.id = new_id,
            };
        } else {
            //error!("layer{} not found!", layer);
        }
    }

    pub fn visibility(&mut self, layer: usize, visibility: bool) {
        if let Some(mut l) = self.layers.get_mut(layer) {
            l.visibility = visibility
        } else {
            //error!("layer{} not found!", layer);
        }
    }

    pub fn get_layer_id(&self, name: &str) -> usize {
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.name.eq(name) {
                return i;
            }
        }
        99
    }

    pub fn get_layer_name(&self, layer: usize) -> &str {
        if let Some(layer) = self.layers.get(layer as usize) {
            &layer.name
        } else {
            //error!("layer{} not found!", layer);
            ""
        }
    }

    pub fn get_id_at_position(&self, layer: usize, position: Vec2) -> Option<u32> {
        let mut pos_x = position.x() as i32;
        let mut pos_y = position.y() as i32;
        if pos_x % 8 != 0 {
            pos_x -= pos_x % 8;
        }
        if pos_y % 8 != 0 {
            pos_y -= pos_y % 8;
        }
        let x = pos_x as i32 / self.tile_width;
        let y = pos_y as i32 / self.tile_height;
        self.get_id_at(layer, x as usize, y as usize)
    }

    pub fn get_id_at(&self, layer_nr: usize, x: usize, y: usize) -> Option<u32> {
        match self.layers.get(layer_nr) {
            None => None,
            Some(layer) => match layer.tiles.get(x, y) {
                None => None,
                Some(tile) => Some(tile.id),
            },
        }
    }

    fn is_inside_viewport(&self, position: Vec2) -> bool {
        !(position.x() < self.viewport.x
            || position.y() < self.viewport.y
            || position.x() > self.viewport.x + self.viewport.w
            || position.y() > self.viewport.y + self.viewport.h)
    }
    pub fn get_clip_from_id(&self, id: u32) -> Rect {
        self.tile_rectangles[&id]
    }

    pub fn get_frames_from_ids(&self, ids: &[u32]) -> Vec<Rect> {
        let mut frames = Vec::with_capacity(ids.len());
        for id in ids {
            frames.push(self.tile_rectangles[&(id)]);
        }
        frames
    }

    fn create_tiles_from_map(&mut self, list: &[Vec<u32>]) -> VecGrid<Tile> {
        let mut tiles = VecGrid::new(list.len(), list[0].len());
        for (x, row) in list.iter().enumerate() {
            for (y, id) in row.iter().enumerate() {
                tiles.set(
                    Tile {
                        id: *id,
                        x: x as i32,
                        y: y as i32,
                        position_x: (x as i32 * self.tile_width) as f32,
                        position_y: (y as i32 * self.tile_height) as f32,
                        ..Tile::default()
                    },
                    x,
                    y,
                );
            }
        }
        tiles
    }

    fn add_layer(&mut self, tiles: VecGrid<Tile>) {
        let layer = Layer {
            tiles,
            ..Layer::default()
        };
        self.layers.push(layer);
    }
    pub fn draw(&self, texture: Texture2D, position: Vec2, layer_to_draw: Option<usize>) {
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.visibility && layer_to_draw.is_none() || layer_to_draw.is_some() && i == layer_to_draw.unwrap() {
                for tile in layer.tiles.get_data().iter().filter(|t| t.is_some()) {
                    match tile {
                        None => (),
                        Some(tile) => {
                            let tmp_pos = Vec2::new(position.x() + tile.position_x, position.y() + tile.position_y);
                            draw_texture_ex(
                                texture,
                                tmp_pos.x(),
                                tmp_pos.y(),
                                layer.color,
                                DrawTextureParams {
                                    dest_size: Some(tile.dest_size),
                                    source: Some(self.tile_rectangles[&tile.id]),
                                    rotation: tile.rotation,
                                    pivot: None,
                                },
                            );
                            if DEBUG {
                                draw_rectangle_lines(tmp_pos.x(), tmp_pos.y(), 8.0, 8.0, 0.1, GREEN);
                                //draw_circle(tmp_pos.x(), tmp_pos.y(),0.5, RED); //low fps
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub struct Tilemap {
    width: usize,
    height: usize,
    viewport: Rect,
    tile_height: i32,
    tile_width: i32,
    layers: Vec<Layer>,
    tile_rectangles: HashMap<u32, Rect>,
    layer_to_draw: i64,
}

#[derive(Debug)]
pub struct Layer {
    tiles: VecGrid<Tile>,
    name: String,
    visibility: bool,
    color: Color,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tile {
    id: u32,
    x: i32,
    y: i32,
    position_x: f32,
    position_y: f32,
    rotation: f32,
    dest_size: Vec2,
}

fn get_tile_rectangles(clip: Rect, tile_width: i32, tile_height: i32) -> HashMap<u32, Rect> {
    let mut id = 0;
    let x = clip.h as i32 / tile_width;
    let y = clip.w as i32 / tile_height;
    let mut tile_rectangles: HashMap<u32, Rect> = HashMap::with_capacity((x * y) as usize);
    for i in 0..x {
        for j in 0..y {
            let rec = Rect::new(
                clip.x + (j * tile_width) as f32,
                clip.y + (i * tile_height) as f32,
                tile_width as f32,
                tile_height as f32,
            );
            tile_rectangles.insert(id, rec);
            id += 1;
        }
    }
    tile_rectangles
}

fn transform_pyxeltilemap(clip: Rect, pyxeltilemap: PyxelTilemap) -> Tilemap {
    Tilemap {
        width: pyxeltilemap.tileswide as usize,
        height: pyxeltilemap.tileshigh as usize,
        viewport: DEFAULT_RECTANGLE,
        tile_height: pyxeltilemap.tile_height,
        tile_width: pyxeltilemap.tile_width,
        layers: transform_pyxellayer(
            &pyxeltilemap.layers,
            pyxeltilemap.tileswide as usize,
            pyxeltilemap.tileshigh as usize,
        ),
        tile_rectangles: get_tile_rectangles(clip, pyxeltilemap.tile_width, pyxeltilemap.tile_height),
        layer_to_draw: DEFAULT_LAYER_TO_DRAW,
    }
}

fn transform_pyxellayer(pyxellayers: &[pyxeledit::Layers], width: usize, height: usize) -> Vec<Layer> {
    let mut layers: Vec<Layer> = Vec::with_capacity(pyxellayers.len());
    for pyxellayer in pyxellayers.iter().rev() {
        let l = Layer {
            tiles: transform_pyxeltile(&pyxellayer.tiles, width, height),
            name: pyxellayer.name.clone(),
            ..Layer::default()
        };
        layers.push(l);
    }
    layers
}

fn transform_pyxeltile(pyxeltiles: &[pyxeledit::Tile], width: usize, height: usize) -> VecGrid<Tile> {
    let mut vecgrid: VecGrid<Tile> = VecGrid::new(width, height);
    for t in pyxeltiles.iter() {
        let tile = Tile {
            id: t.id as u32,
            x: t.x,
            y: t.y,
            position_x: t.position_x.unwrap(),
            position_y: t.position_y.unwrap(),
            rotation: t.rotation.unwrap(),
            dest_size: vec2(t.dest_size.unwrap().0, t.dest_size.unwrap().1),
        };
        vecgrid.set(tile, t.x as usize, t.y as usize);
    }
    vecgrid
}
/*
fn draw_everything(rectangle: &Rect) -> bool{
    let rectangle_to_compare = DEFAULT_RECTANGLE;
    rectangle_to_compare.eq(rectangle)
}
 */

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            tiles: VecGrid::new(1, 1),
            name: "".to_string(),
            visibility: true,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            id: 0,
            x: 0,
            y: 0,
            position_x: 0.0,
            position_y: 0.0,
            rotation: 0.0,
            dest_size: vec2(0.0, 0.0),
        }
    }
}

const DEFAULT_RECTANGLE: Rect = Rect {
    x: 0.0,
    y: 0.0,
    w: 0.0,
    h: 0.0,
};
const DEFAULT_LAYER_TO_DRAW: i64 = -1;
