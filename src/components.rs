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
}

impl From<Speed> for ffi::Vector2 {
    fn from(speed: Speed) -> Self {
        Self {
            x: speed.x,
            y: speed.y,
        }
    }
}

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

impl From<CircCollider> for ffi::Vector2 {
    fn from(collider: CircCollider) -> Self {
        Self {
            x: collider.val.x,
            y: collider.val.y,
        }
    }
}
