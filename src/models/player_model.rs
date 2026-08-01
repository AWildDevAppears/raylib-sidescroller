/**
* Copyright (c) AWildDevAppears
*/
use raylib::ffi::{Rectangle, Vector2};

pub struct PlayerModel {
    pub bounds: Rectangle,
    pub velocity: Vector2,
    pub max_velocity: Vector2,
    pub acceleration: f32,
    pub acceleration_taper: f32,
    pub is_airborne: bool,
}

impl PlayerModel {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            bounds,
            velocity: Vector2 { x: 0.0, y: 0.0 },
            max_velocity: Vector2 { x: 1.0, y: 1.0 },
            acceleration: 0.3,
            acceleration_taper: 0.1,
            is_airborne: false,
        }
    }

    pub fn move_left(&mut self) {
        self.velocity.x = (self.velocity.x - self.acceleration).max(-self.max_velocity.x);
    }

    pub fn move_right(&mut self) {
        self.velocity.x = (self.velocity.x + self.acceleration).min(self.max_velocity.x);
    }

    pub fn decelerate_x(&mut self) {
        self.velocity.x = if self.velocity.x > 0.0 {
            (self.velocity.x - self.acceleration_taper).max(0.0)
        } else {
            (self.velocity.x + self.acceleration_taper).min(0.0)
        };
    }
}
