/**
* Copyright (c) AWildDevAppears
*/
use raylib::{
    RaylibHandle, RaylibThread,
    camera::Camera2D,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::{Color, KeyboardKey, Rectangle, Vector2},
    texture::Texture2D,
};

use crate::{
    constants::CONSTANT_GRAVITY,
    handlers::map_handler::{DrawCall, MapLayers, load_map, load_texture},
    models::player_model::{MovementX, PlayerModel},
};

pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub game_name: String,
    pub layers: Vec<(MapLayers, Vec<DrawCall>)>,
    pub texture: Option<Texture2D>,
    pub player: PlayerModel,
    pub camera: Camera2D,
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
        self.layers = load_map();
        self.texture = Some(load_texture(game, thread));
    }

    pub fn update(&mut self, game: &mut RaylibHandle) {
        if game.is_key_down(KeyboardKey::KEY_D) {
            self.player.move_x(MovementX::RIGHT);
        } else if game.is_key_down(KeyboardKey::KEY_A) {
            self.player.move_x(MovementX::LEFT);
        } else {
            self.player.decelerate_x();
        }

        self.player
            .handle_jump_pressed(game.is_key_pressed(KeyboardKey::KEY_SPACE));

        self.player
            .handle_jump_released(game.is_key_released(KeyboardKey::KEY_SPACE));

        let current_gravity = if self.player.velocity.y.abs() < 1.5 && !self.player.is_grounded {
            CONSTANT_GRAVITY * 0.5
        } else if self.player.velocity.y > 0.0 {
            CONSTANT_GRAVITY * self.player.fall_gravity_mult
        } else {
            CONSTANT_GRAVITY
        };

        self.player.velocity.y += current_gravity;

        self.camera.target = Vector2 {
            x: (self.player.bounds.x - (self.screen_width as f32 / 2.0)).round(),
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
                        self.player.is_grounded = true;
                        self.player.is_double_jumping = false;
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
}
