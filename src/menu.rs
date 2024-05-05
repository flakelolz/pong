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
                *state = GameState::Playing;
                change_opponent(world, Player::Cpu);
            }

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 20., w, h);
            if d.gui_button(main_menu, Some(rstr!("VS P2"))) {
                *state = GameState::Playing;
                change_opponent(world, Player::Right);
            }

            d.draw_text(
                "P1: W - S\n\nP2: Up - Down\n\nEsc: Pause\n",
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
            let (w, h) = (96., 32.);

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) - 50., w, h);
            if d.gui_button(main_menu, Some(rstr!("Main Menu"))) {
                *state = GameState::Starting;
            }

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., FHEIGHT / 2., w, h);
            if d.gui_button(main_menu, Some(rstr!("Reset"))) {
                *state = GameState::Reset;
            }

            let main_menu = Rectangle::new((FWIDTH / 2.) - w / 2., (FHEIGHT / 2.) + 50., w, h);
            if d.gui_button(main_menu, Some(rstr!("Quit Game"))) {
                *quit = true;
            }

            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                *state = GameState::Playing;
            }
        }
        _ => (),
    }
}

pub fn reset_game(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    world: &mut World,
    state: &mut GameState,
) {
    let mut opponent = Player::Cpu;
    for (_, player) in world.query::<&mut Player>().iter() {
        if *player == Player::Cpu || *player == Player::Right {
            opponent = *player;
        }
    }

    world.clear();

    spawn_paddle(rl, thread, world, Player::Left);
    spawn_paddle(rl, thread, world, opponent);
    spawn_ball(rl, thread, world);
    spawn_score(world);
    *state = GameState::Playing;
}
