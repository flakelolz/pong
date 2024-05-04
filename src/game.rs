use crate::prelude::*;

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut world = World::new();
    let mut _commands = CommandBuffer::new();

    let mut camera = Camera2D {
        zoom: 1.0,
        ..Default::default()
    };

    spawn_paddle(rl, thread, &mut world, Player::Left);
    spawn_paddle(rl, thread, &mut world, Player::Cpu);
    spawn_ball(rl, thread, &mut world);
    resources(&mut world);

    while !rl.window_should_close() {
        // Update
        move_paddle(rl, &mut world);
        move_ball(&mut world);
        check_collision(&mut world);
        update_score(&mut world);

        // Camera
        // FIX: Scale with screen size correctly
        let width = rl.get_screen_width() as f32;
        let height = rl.get_screen_height() as f32;
        let zoom = (width / WWIDTH as f32).min(height / WHEIGHT as f32);
        camera.zoom = zoom;

        // Drawing
        let mut d = rl.begin_drawing(thread);
        let mut d = d.begin_mode2D(camera);
        d.clear_background(Color::BLACK);

        d.draw_line(WWIDTH / 2, 0, WWIDTH / 2, WHEIGHT, Color::WHITE);
        render_paddle(&mut d, &world);
        render_ball(&mut d, &world);
        render_score(&mut d, &world);
    }
}
