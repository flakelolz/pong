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

pub struct Collider {
    pub x: f32,
    pub y: f32,
}

impl Collider {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl From<&Collider> for ffi::Vector2 {
    fn from(coll: &Collider) -> Self {
        Self {
            x: coll.x,
            y: coll.y,
        }
    }
}

// impl From<&Collider> for ffi::Rectangle {
//     fn from(coll: &Collider) -> Self {
//         Self {
//             x: coll.rect.x,
//             y: coll.rect.y,
//             width: coll.rect.width,
//             height: coll.rect.height,
//         }
//     }
// }

pub struct BallCollider {
    pub x: f32,
    pub y: f32,
    pub circle: Vector2,
}

impl BallCollider {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            circle: Vector2::new(x / 2., y / 2.),
        }
    }
}

impl From<&BallCollider> for ffi::Vector2 {
    fn from(coll: &BallCollider) -> Self {
        Self {
            x: coll.circle.x,
            y: coll.circle.y,
        }
    }
}
