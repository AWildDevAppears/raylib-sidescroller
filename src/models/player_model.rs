/**
* Copyright (c) AWildDevAppears
*/
use raylib::ffi::Vector2;

pub struct PlayerModel {
    pub position: Vector2,
    pub velocity: Vector2,
    pub max_velocity: Vector2,
}

impl PlayerModel {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            velocity: Vector2 { x: 0.0, y: 0.0 },
            max_velocity: Vector2 { x: 1.0, y: 1.0 },
        }
    }
}
