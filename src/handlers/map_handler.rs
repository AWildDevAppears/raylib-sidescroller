/**
* Copyright (c) AWildDevAppears
*/
use raylib::{RaylibHandle, RaylibThread, ffi::Rectangle, texture::Texture2D};
use tiled::{Layer, Loader};

pub enum MapLayers {
    Unbound,
    Floor,
    Background,
    Flag,
    Bricks,
    Pipes,
    PowerBlocks,
}

pub struct DrawCall {
    pub source: Rectangle,
    pub dest: Rectangle,
}

pub fn load_texture(game: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    game.load_texture(thread, "assets/28-touch.png")
        .expect("Failed to load textures")
}

pub fn load_map() -> Vec<(MapLayers, Vec<DrawCall>)> {
    let mut loader = Loader::new();

    let map = match loader.load_tmx_map("assets/level-one.tmx") {
        Ok(m) => m,
        Err(e) => panic!("{}", e),
    };
    let tile_width = map.tile_width as f32;
    let tile_height = map.tile_height as f32;

    let mut layers = vec![];

    for (_, m) in map.layers().enumerate() {
        println!("{:?}", m.name);

        let name = match m.name.as_str() {
            "floor" => MapLayers::Floor,
            "background" => MapLayers::Background,
            "flag" => MapLayers::Flag,
            "bricks" => MapLayers::Bricks,
            "pipes" => MapLayers::Pipes,
            "power_blocks" => MapLayers::PowerBlocks,
            _ => MapLayers::Unbound,
        };

        layers.push((
            name,
            parse_layer(m, (map.width, map.height), (tile_width, tile_height)),
        ));
    }

    layers
}

fn parse_layer(layer: Layer<'_>, size: (u32, u32), tile_size: (f32, f32)) -> Vec<DrawCall> {
    let (width, height) = size;
    let (tile_width, tile_height) = tile_size;

    let mut rects = Vec::new();
    if let Some(tile_layer) = layer.as_tile_layer() {
        for x in 0..width {
            for y in 0..height {
                if let Some(layer_tile) = tile_layer.get_tile(x as i32, y as i32) {
                    if let Some(tile) = layer_tile.get_tile() {
                        let tileset = tile.tileset();
                        let tile_id = layer_tile.id();
                        let columns = tileset.columns;

                        let src_x = (tile_id % columns) * tileset.tile_width;
                        let src_y = (tile_id / columns) * tileset.tile_height;

                        rects.push(DrawCall {
                            source: Rectangle::new(
                                src_x as f32,
                                src_y as f32,
                                tile_width,
                                tile_height,
                            ),
                            dest: Rectangle::new(
                                x as f32 * tile_width,
                                y as f32 * tile_height,
                                tile_width,
                                tile_height,
                            ),
                        });
                    }
                }
            }
        }
    }

    rects
}
