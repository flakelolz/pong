use crate::prelude::*;

pub fn handle_menus(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    world: &mut World,
    state: &mut GameState,
    quit: &mut bool,
) {
    let (w, h) = (96., 32.);

    match state {
        GameState::Starting => {
            let mm_vs_cpu = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 50., w, h);
            if d.gui_button(mm_vs_cpu, Some(rstr!("VS CPU"))) {
                *state = GameState::Playing;
                change_opponent(world, Player::Cpu);
                reset_game(world, state);
            }

            let mm_vs_p2 = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 20., w, h);
            if d.gui_button(mm_vs_p2, Some(rstr!("VS P2"))) {
                *state = GameState::Playing;
                change_opponent(world, Player::Right);
                reset_game(world, state);
            }

            d.draw_text(
                "P1: W - S\n\nP2: Up - Down\n\nEnter: Pause\n",
                12,
                HEIGHT - 120,
                20,
                Color::WHITE,
            );
        }
        GameState::Playing => {
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                *state = GameState::Paused;
            }
        }
        GameState::Paused => {
            let p_main_menu = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 50., w, h);
            if d.gui_button(p_main_menu, Some(rstr!("Main Menu"))) {
                *state = GameState::Starting;
            }

            let p_reset = rrect((FWIDTH / 2.) - w / 2., FHEIGHT / 2., w, h);
            if d.gui_button(p_reset, Some(rstr!("Reset"))) {
                *state = GameState::Reset;
            }

            let p_quit = rrect((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 50., w, h);
            if d.gui_button(p_quit, Some(rstr!("Quit Game"))) {
                *quit = true;
            }

            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                *state = GameState::Playing;
            }
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
