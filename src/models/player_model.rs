/**
* Copyright (c) AWildDevAppears
*/
use raylib::ffi::{Rectangle, Vector2};

#[repr(u8)]
pub enum MovementX {
    LEFT,
    RIGHT,
}

pub struct PlayerModel {
    pub bounds: Rectangle,
    pub velocity: Vector2,
    pub max_velocity: Vector2,
    pub acceleration: f32,
    pub acceleration_taper: f32,
    pub is_grounded: bool,
    pub fall_gravity_mult: f32,
    pub can_double_jump: bool,
    pub is_double_jumping: bool,
}

impl PlayerModel {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            bounds,
            velocity: Vector2 { x: 0.0, y: 0.0 },
            max_velocity: Vector2 { x: 1.0, y: 1.0 },
            acceleration: 0.2,
            acceleration_taper: 0.1,
            is_grounded: false,
            fall_gravity_mult: 1.0,
            can_double_jump: false,
            is_double_jumping: false,
        }
    }

    pub fn move_x(&mut self, direction: MovementX) {
        match direction {
            MovementX::LEFT => self.move_left(),
            MovementX::RIGHT => self.move_right(),
        }
    }

    pub fn decelerate_x(&mut self) {
        self.velocity.x = if self.velocity.x > 0.0 {
            (self.velocity.x - self.acceleration_taper).max(0.0)
        } else {
            (self.velocity.x + self.acceleration_taper).min(0.0)
        };
    }

    pub fn handle_jump_pressed(&mut self, jumped: bool) {
        if jumped {
            if self.is_grounded {
                self.velocity.y = -1.25;
                self.is_grounded = false;
            }

            if self.can_double_jump {
                self.can_double_jump = false;
                self.is_double_jumping = true;
                self.velocity.y = -1.25;
            }
        }
    }

    pub fn handle_jump_released(&mut self, released: bool) {
        if released {
            if self.velocity.y < 0.0 && !self.is_double_jumping {
                self.velocity.y *= 0.5;
                self.can_double_jump = true;
            }
        }
    }

    fn move_left(&mut self) {
        self.velocity.x = (self.velocity.x - self.acceleration).max(-self.max_velocity.x);
    }

    fn move_right(&mut self) {
        self.velocity.x = (self.velocity.x + self.acceleration).min(self.max_velocity.x);
    }
}
