use std::cmp::max;

use raylib::{
    RaylibHandle, RaylibThread,
    camera::Camera2D,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::{Color, KeyboardKey, Rectangle, Vector2},
    texture::Texture2D,
};
use tiled::{Layer, Loader};

use crate::models::player_model::PlayerModel;

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
    pub camera: Camera2D,
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
            player: PlayerModel::new(Rectangle {
                x: 0.0,
                y: screen_height as f32 - (8.0 * 32.0), // TODO: Calculate this position from the
                // map.
                width: 32.0,
                height: 32.0,
            }),
            camera: Camera2D {
                offset: Vector2 { x: 0.0, y: 0.0 },
                target: Vector2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                zoom: 1.0,
            },
        }
    }

    pub fn preload(&mut self, game: &mut RaylibHandle, thread: &RaylibThread) {
        self.load_map();

        self.texture = Some(self.load_texture(game, thread));
    }

    pub fn update(&mut self, game: &mut RaylibHandle) {
        if game.is_key_down(KeyboardKey::KEY_D) {
            self.player.velocity.x = (self.player.velocity.x + self.player.accelleration)
                .min(self.player.max_velocity.x);
        } else if game.is_key_down(KeyboardKey::KEY_A) {
            self.player.velocity.x = (self.player.velocity.x - self.player.accelleration)
                .max(-self.player.max_velocity.x);
        } else {
            self.player.velocity.x = if self.player.velocity.x > 0.0 {
                (self.player.velocity.x - self.player.accelleration_taper).max(0.0)
            } else {
                (self.player.velocity.x + self.player.accelleration_taper).min(0.0)
            };
        }

        // TODO: Jump

        // TODO: Camera bounds
        self.camera.target = Vector2 {
            x: self.player.bounds.x - (self.screen_width as f32 / 2.0),
            y: 0.0,
        };

        // TODO: Camera bounds for side walls

        self.player.bounds.x += self.player.velocity.x;

        'x_loop: for (_group, layer) in &self.layers {
            for call in layer {
                if let Some(_overlap) = self.player.bounds.get_collision_rec(call.dest) {
                    if self.player.velocity.x > 0.0 {
                        self.player.bounds.x = call.dest.x - self.player.bounds.width;
                        self.player.velocity.x = 0.0;
                        break 'x_loop;
                    } else if self.player.velocity.x < 0.0 {
                        self.player.bounds.x = call.dest.x + call.dest.width;
                        self.player.velocity.x = 0.0;
                        break 'x_loop;
                    }
                }
            }
        }

        self.player.bounds.y += self.player.velocity.y;

        'y_loop: for (_group, layer) in &self.layers {
            for call in layer {
                if let Some(_overlap) = self.player.bounds.get_collision_rec(call.dest) {
                    if self.player.velocity.y > 0.0 {
                        self.player.bounds.y = call.dest.y - self.player.bounds.height;
                        self.player.velocity.y = 0.0;
                        break 'y_loop;
                    } else if self.player.velocity.y < 0.0 {
                        self.player.bounds.y = call.dest.y + call.dest.height;
                        self.player.velocity.y = 0.0;
                        break 'y_loop;
                    }
                }
            }
        }
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

        draw.draw_rectangle_pro(
            self.player.bounds,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
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
