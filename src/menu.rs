use crate::prelude::*;

pub fn handle_menus(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    world: &mut World,
    state: &mut GameState,
    quit: &mut bool,
    settings: &mut Settings,
) {
    let (w, h) = (96., 32.);

    match state {
        GameState::Starting => {
            // Vs CPU button
            let mm_vs_cpu = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 50., w, h);
            if d.gui_button(mm_vs_cpu, Some(c"VS CPU")) {
                *state = GameState::Playing;
                change_opponent(world, Player::Cpu);
                reset_game(world, state);
            }

            // Vs P2 button
            let mm_vs_p2 = rrect((FWIDTH / 2.) - w / 2., FHEIGHT / 2., w, h);
            if d.gui_button(mm_vs_p2, Some(c"VS P2")) {
                *state = GameState::Playing;
                change_opponent(world, Player::Right);
                reset_game(world, state);
            }

            // Quit game
            let p_quit = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 50., w, h);
            if d.gui_button(p_quit, Some(c"Quit Game")) {
                *quit = true;
            }

            // Controls help
            d.gui_label(
                rrect(12., FHEIGHT - 65., 100., 30.),
                Some(c"P1: W - S\n\nP2: Up - Down\n\nEnter: Pause\n"),
            );

            // Volume slider
            d.gui_slider_bar(
                rrect((FWIDTH) - 150., FHEIGHT - 40., 100., 10.),
                Some(c"Volume"),
                Some(rstr!("{}", settings.volume).as_c_str()),
                &mut settings.volume,
                0.0,
                1.0,
            );
        }
        GameState::Playing => {
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                *state = GameState::Paused;
            }
        }
        GameState::Paused => {
            // Unpause
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                *state = GameState::Playing;
            }

            // Resume
            let p_main_menu = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 100., w, h);
            if d.gui_button(p_main_menu, Some(c"Resume")) {
                *state = GameState::Playing;
            }

            // Reset game
            let p_reset = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 50., w, h);
            if d.gui_button(p_reset, Some(c"Reset")) {
                *state = GameState::Reset;
            }

            // Back to main menu
            let p_main_menu = rrect((FWIDTH / 2.) - w / 2., FHEIGHT / 2., w, h);
            if d.gui_button(p_main_menu, Some(c"Main Menu")) {
                *state = GameState::Starting;
            }

            // Quit game
            let p_quit = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 50., w, h);
            if d.gui_button(p_quit, Some(c"Quit Game")) {
                *quit = true;
            }

            // Controls help
            d.gui_label(
                rrect(12., FHEIGHT - 65., 100., 30.),
                Some(c"P1: W - S\n\nP2: Up - Down\n\nEnter: Pause\n"),
            );

            // Volume slider
            d.gui_slider_bar(
                rrect((FWIDTH) - 150., FHEIGHT - 40., 100., 10.),
                Some(c"Volume"),
                Some(rstr!("{}", settings.volume).as_c_str()),
                &mut settings.volume,
                0.0,
                1.0,
            );
        }
        _ => (),
    }
}

pub fn reset_game(world: &mut World, state: &mut GameState) {
    reset_paddles(world);
    reset_ball(world);
    reset_score(world);
    *state = GameState::Playing;
}
