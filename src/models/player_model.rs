/**
* Copyright (c) AWildDevAppears
*/
use raylib::ffi::{Rectangle, Vector2};

pub struct PlayerModel {
    pub bounds: Rectangle,
    pub velocity: Vector2,
    pub max_velocity: Vector2,
    pub accelleration: f32,
    pub accelleration_taper: f32,
}

impl PlayerModel {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            bounds,
            velocity: Vector2 { x: 0.0, y: 0.0 },
            max_velocity: Vector2 { x: 1.0, y: 1.0 },
            accelleration: 0.3,
            accelleration_taper: 0.1,
        }
    }
}
