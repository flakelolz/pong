use crate::prelude::*;

pub fn handle_menus(
    d: &mut RaylibMode2D<RaylibDrawHandle>,
    world: &mut World,
    state: &mut GameState,
    quit: &mut bool,
) {
    match state {
        GameState::Starting => {
            let (w, h) = (96., 32.);

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 50., w, h);
            if d.gui_button(main_menu, Some(rstr!("VS CPU"))) {
                // *opponent = Player::Cpu;
                *state = GameState::Playing;
                change_opponent(world, Player::Cpu);
            }

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 20., w, h);
            if d.gui_button(main_menu, Some(rstr!("VS P2"))) {
                // *opponent = Player::Right;
                *state = GameState::Playing;
                change_opponent(world, Player::Right);
            }
        }
        GameState::Playing => {
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                *state = GameState::Paused;
            }
        }
        GameState::Paused => {
            let (w, h) = (96., 32.);

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 50., w, h);
            if d.gui_button(main_menu, Some(rstr!("Main Menu"))) {
                *state = GameState::Starting;
            }

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 20., w, h);
            if d.gui_button(main_menu, Some(rstr!("Quit Game"))) {
                *quit = true;
            }

            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                *state = GameState::Playing;
            }
        }
    }
}
