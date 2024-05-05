use crate::prelude::*;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Starting,
    Playing,
    Paused,
}

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut world = World::new();
    let mut _commands = CommandBuffer::new();

    let mut camera = Camera2D {
        zoom: 1.0,
        ..Default::default()
    };

    let mut state = GameState::Starting;

    spawn_paddle(rl, thread, &mut world, Player::Left);
    spawn_paddle(rl, thread, &mut world, Player::Right);
    spawn_ball(rl, thread, &mut world);
    spawn_score(&mut world);

    let mut quit = false;
    let mut exit_window = false;

    while !exit_window {
        if rl.window_should_close() || quit {
            exit_window = true;
        }
        // Update
        if state == GameState::Playing {
            move_paddle(rl, &mut world);
            move_ball(&mut world);
            ball_collision(&mut world);
            update_score(&mut world);
        }

        // Camera
        // FIX: Scale with screen size correctly
        let width = rl.get_screen_width() as f32;
        let height = rl.get_screen_height() as f32;
        let zoom = (width / FWIDTH).min(height / FHEIGHT);
        camera.zoom = zoom;

        // Drawing
        let mut d = rl.begin_drawing(thread);
        let mut d = d.begin_mode2D(camera);
        d.clear_background(Color::BLACK);

        if state == GameState::Playing {
            d.draw_line(WIDTH / 2, 0, WIDTH / 2, HEIGHT, Color::WHITE);
            render_paddle(&mut d, &world);
            render_ball(&mut d, &world);
            render_score(&mut d, &world);
        }
        handle_menus(&mut d, &mut world, &mut state, &mut quit);
    }
}
