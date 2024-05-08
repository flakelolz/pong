#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod assets;
mod ball;
mod components;
mod game;
mod menu;
mod paddle;
mod score;
mod settings;

pub mod prelude {
    pub use crate::assets::*;
    pub use crate::ball::*;
    pub use crate::components::*;
    pub use crate::game::*;
    pub use crate::menu::*;
    pub use crate::paddle::*;
    pub use crate::score::*;
    pub use crate::settings::*;

    pub use hecs::{CommandBuffer, Entity, World};
    pub use include_dir::{include_dir, Dir};
    pub use raylib::prelude::*;
    pub use std::rc::Rc;

    pub static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

    pub const WIDTH: i32 = 800;
    pub const HEIGHT: i32 = 450;
    pub const FWIDTH: f32 = WIDTH as f32;
    pub const FHEIGHT: f32 = HEIGHT as f32;
}

use prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .resizable()
        .title("Pong")
        .vsync()
        .build();

    #[cfg(not(debug_assertions))]
    rl.set_exit_key(None);

    let audio = RaylibAudio::init_audio_device().unwrap();

    let mut target = rl
        .load_render_texture(&thread, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    game::game(&mut rl, &thread, audio, &mut target);
}
