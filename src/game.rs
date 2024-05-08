#![allow(clippy::field_reassign_with_default)]
use crate::prelude::*;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Starting,
    Reset,
    Playing,
    Paused,
}

pub fn game(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    audio: RaylibAudio,
    target: &mut RenderTexture2D,
) {
    let mut world = World::new();
    let audio = Rc::new(audio);
    let mut settings = Settings::default();

    let assets = Assets::load_assets(audio.as_ref());

    let mut state = GameState::Starting;

    spawn_paddle(rl, thread, &mut world, Player::Left);
    spawn_paddle(rl, thread, &mut world, Player::Right);
    spawn_ball(rl, thread, &mut world);
    spawn_score(&mut world);

    let mut quit = false;
    let mut exit_window = false;

    while !exit_window {
        // Exit game conditions
        if rl.window_should_close() || quit {
            exit_window = true;
        }

        // audio.set_master_volume(volume);
        apply_settings(&settings, audio.clone());

        /* --- Update --- */
        if state == GameState::Playing {
            move_paddle(rl, &mut world);
            move_ball(rl, &mut world, &assets.walls);
            ball_collision(&mut world, &assets.bounce);
            update_score(rl, &mut world, &assets.scores);
        }

        if state == GameState::Reset {
            reset_game(&mut world, &mut state);
        }

        // Calculate window
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let scale = (width / WIDTH).min(height / HEIGHT) as f32;

        // Mouse scale
        rl.set_mouse_scale(1.0 / scale, 1.0 / scale);
        rl.set_mouse_offset(rvec2(
            -(rl.get_screen_width() as f32 - (FWIDTH * scale)) * 0.5,
            -(rl.get_screen_height() as f32 - (FHEIGHT * scale)) * 0.5,
        ));

        /* --- Drawing --- */
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        {
            // Render to texture
            let mut d = d.begin_texture_mode(thread, target);
            d.clear_background(Color::BLACK);

            if state == GameState::Playing {
                d.draw_line(WIDTH / 2, 0, WIDTH / 2, HEIGHT, Color::WHITE);
                render_paddle(&mut d, &world);
                render_ball(&mut d, &world);
                render_score(&mut d, &world);
            }

            handle_menus(&mut d, &mut world, &mut state, &mut quit, &mut settings);
        }

        // Render texture to screen with proper scaling
        d.draw_texture_pro(
            target.texture(),
            rrect(0.0, 0.0, target.texture.width, -target.texture.height),
            rrect(
                (d.get_screen_width() as f32 - (FWIDTH * scale)) * 0.5,
                (d.get_screen_height() as f32 - (FHEIGHT * scale)) * 0.5,
                FWIDTH * scale,
                FHEIGHT * scale,
            ),
            rvec2(0, 0),
            0.0,
            Color::WHITE,
        );
    }
}
