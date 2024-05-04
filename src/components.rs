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
