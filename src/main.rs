mod ball;
mod components;
mod game;
mod paddle;
mod utils;

pub mod prelude {
    pub use crate::ball::*;
    pub use crate::components::*;
    pub use crate::paddle::*;

    pub use hecs::{CommandBuffer, Entity, World};
    pub use raylib::prelude::*;

    pub const WWIDTH: i32 = 800;
    pub const WHEIGHT: i32 = 450;
}
use prelude::*;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WWIDTH, WHEIGHT)
        .resizable()
        .title("Raylib")
        .build();

    rl.set_target_fps(60);

    game::game(&mut rl, &thread);
}
