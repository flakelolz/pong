#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod assets;
mod ball;
mod components;
mod game;
mod menu;
mod paddle;
mod score;

pub mod prelude {
    pub use crate::assets::*;
    pub use crate::ball::*;
    pub use crate::components::*;
    pub use crate::game::*;
    pub use crate::menu::*;
    pub use crate::paddle::*;
    pub use crate::score::*;

    pub use hecs::{CommandBuffer, Entity, World};
    pub use raylib::prelude::*;

    pub const WIDTH: i32 = 800;
    pub const HEIGHT: i32 = 450;
    pub const FWIDTH: f32 = 800.;
    pub const FHEIGHT: f32 = 450.;
}
use prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .resizable()
        .title("Pong")
        .build();

    rl.set_target_fps(60);
    // rl.set_exit_key(None);
    let mut audio = RaylibAudio::init_audio_device().unwrap();
    audio.set_master_volume(0.65);

    game::game(&mut rl, &thread, &mut audio);
}
