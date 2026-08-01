use std::os::unix::thread;

use raylib::{
    RaylibHandle, RaylibThread,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::{CSSPalette, Color, KeyboardKey, Rectangle, Vector2},
    math::lerp,
    texture::Texture2D,
};
use tiled::{Layer, Loader};

use crate::{gamestate::MapLayers::Background, models::player_model::PlayerModel};

/**
* Copyright (c) AWildDevAppears
*/

pub struct DrawCall {
    pub source: Rectangle,
    pub dest: Rectangle,
}

pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub game_name: String,
    pub layers: Vec<(MapLayers, Vec<DrawCall>)>,
    pub texture: Option<Texture2D>,
    pub player: PlayerModel,
}

pub enum MapLayers {
    Unbound,
    Floor,
    Background,
    Flag,
    Bricks,
    Pipes,
    PowerBlocks,
}

impl GameState {
    pub fn new() -> Self {
        let screen_width = 640;
        let screen_height = 640;

        Self {
            screen_width,
            screen_height,
            game_name: "Boilerplate".to_string(),
            layers: vec![],
            texture: None,
            player: PlayerModel::new(Vector2 {
                x: 0.0,
                y: screen_height as f32 - (8.0 * 32.0), // TODO: Calculate this position from the
                                                        // map.
            }),
        }
    }

    pub fn preload(&mut self, game: &mut RaylibHandle, thread: &RaylibThread) {
        self.load_map();

        self.texture = Some(self.load_texture(game, thread));
    }

    pub fn update(&mut self, game: &mut RaylibHandle) {
        // TODO: LERPed velocity
        if game.is_key_down(KeyboardKey::KEY_D) {
            self.player.velocity = Vector2 {
                x: lerp(self.player.velocity.x, self.player.max_velocity.x, 1.0),
                y: self.player.velocity.y,
            }
        }

        if game.is_key_down(KeyboardKey::KEY_A) {
            self.player.velocity = Vector2 {
                x: lerp(self.player.velocity.x, -self.player.max_velocity.x, 1.0),
                y: self.player.velocity.y,
            }
        }

        // TODO: Jump

        self.player.position += self.player.velocity;
        self.player.velocity = Vector2 { x: 0.0, y: 0.0 };

        // TODO: Camera chase

        // TODO: Collision groups
    }

    pub fn draw(&self, draw: &mut RaylibDrawHandle) {
        if let Some(ref tex) = self.texture {
            for (_variant, layer) in &self.layers {
                for call in layer {
                    draw.draw_texture_pro(
                        tex,
                        call.source,
                        call.dest,
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }

        draw.draw_rectangle_v(
            self.player.position,
            Vector2 { x: 32.0, y: 32.0 },
            Color::YELLOW,
        );
    }

    fn load_texture(&self, game: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
        game.load_texture(thread, "assets/28-touch.png")
            .expect("Failed to load textures")
    }

    fn load_map(&mut self) {
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
                self.parse_layer(m, (map.width, map.height), (tile_width, tile_height)),
            ));
        }

        self.layers = layers;
    }

    fn parse_layer(
        &self,
        layer: Layer<'_>,
        size: (u32, u32),
        tile_size: (f32, f32),
    ) -> Vec<DrawCall> {
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
}
