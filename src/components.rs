use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Position> for ffi::Vector2 {
    fn from(pos: Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<&Position> for ffi::Vector2 {
    fn from(pos: &Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

impl Speed {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn player_speed() -> Self {
        Self { x: 0., y: 600. }
    }

    pub fn cpu_speed() -> Self {
        Self { x: 0., y: 380. }
    }
}

impl From<Speed> for ffi::Vector2 {
    fn from(speed: Speed) -> Self {
        Self {
            x: speed.x,
            y: speed.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RectCollider {
    pub val: Rectangle,
}

impl RectCollider {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            val: Rectangle::new(x, y, w, h),
        }
    }
}

impl From<RectCollider> for ffi::Rectangle {
    fn from(collider: RectCollider) -> Self {
        Self {
            x: collider.val.x,
            y: collider.val.y,
            width: collider.val.width,
            height: collider.val.height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircCollider {
    pub val: Vector2,
    pub radius: f32,
}

impl CircCollider {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            val: Vector2::new(x, y),
            radius: (x / 2.).max(y / 2.),
        }
    }
}
