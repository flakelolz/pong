use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Resources {
    pub score: Score
}

#[derive(Debug, Default)]
pub struct Score {
    pub left: i32,
    pub right: i32
}

pub fn resources(world: &mut World) -> Entity {
    world.spawn((Resources::default(),))
}
