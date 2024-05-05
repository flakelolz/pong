use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Left,
    Right,
    Cpu,
}

pub fn spawn_paddle(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    world: &mut World,
    player: Player,
) -> Entity {
    let image = Image::load_image("assets/textures/paddle.png").unwrap();

    let (paddle_w, paddle_h) = (24., 96.);

    match player {
        Player::Left => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            let (paddle_x, paddle_y) = (20., FHEIGHT / 2.);
            world.spawn((
                Player::Left,
                Position::new(paddle_x, paddle_y),
                RectCollider::new(
                    paddle_x - paddle_w / 2.,
                    paddle_y - paddle_h / 2.,
                    paddle_w,
                    paddle_h,
                ),
                Speed::player_speed(),
                texture,
            ))
        }
        Player::Right => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            let (paddle_x, paddle_y) = (FWIDTH - 20., FHEIGHT / 2.);
            world.spawn((
                Player::Right,
                Position::new(paddle_x, paddle_y),
                RectCollider::new(
                    paddle_x - paddle_w / 2.,
                    paddle_y - paddle_h / 2.,
                    paddle_w,
                    paddle_h,
                ),
                Speed::player_speed(),
                texture,
            ))
        }
        Player::Cpu => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            let (paddle_x, paddle_y) = (FWIDTH - 20., FHEIGHT / 2.);
            world.spawn((
                Player::Cpu,
                Position::new(paddle_x, paddle_y),
                RectCollider::new(
                    paddle_x - paddle_w / 2.,
                    paddle_y - paddle_h / 2.,
                    paddle_w,
                    paddle_h,
                ),
                Speed::cpu_speed(),
                texture,
            ))
        }
    }
}

pub fn move_paddle(rl: &RaylibHandle, world: &mut World) {
    for (_, (player, pos, collider, speed)) in world
        .query::<(&Player, &mut Position, &mut RectCollider, &Speed)>()
        .iter()
    {
        // Limit movement
        if pos.y - collider.val.height / 2. <= 0. {
            pos.y = collider.val.height / 2.;
        }
        if pos.y + collider.val.height / 2. >= FHEIGHT {
            pos.y = FHEIGHT - collider.val.height / 2.;
        }

        match player {
            Player::Left => {
                if rl.is_key_down(KeyboardKey::KEY_W) {
                    pos.y -= speed.y;
                } else if rl.is_key_down(KeyboardKey::KEY_S) {
                    pos.y += speed.y;
                }
            }
            Player::Right => {
                if rl.is_key_down(KeyboardKey::KEY_UP) {
                    pos.y -= speed.y;
                } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                    pos.y += speed.y;
                }
            }
            Player::Cpu => {
                if let Some((_, (_, ball_pos))) = world.query::<(&Ball, &Position)>().iter().next()
                {
                    if pos.y > ball_pos.y {
                        pos.y -= speed.y;
                    } else if pos.y < ball_pos.y {
                        pos.y += speed.y;
                    }
                }
            }
        }

        // Update collider
        collider.val.x = pos.x - collider.val.width / 2.;
        collider.val.y = pos.y - collider.val.height / 2.;
    }
}

pub fn change_opponent(world: &mut World, opponent: Player) {
    for (_, (player, speed)) in world.query::<(&mut Player, &mut Speed)>().iter() {
        if *player == Player::Cpu || *player == Player::Right {
            match opponent {
                Player::Right => {
                    *player = Player::Right;
                    *speed = Speed::player_speed();
                }
                Player::Cpu => {
                    *player = Player::Cpu;
                    *speed = Speed::cpu_speed();
                }
                _ => (),
            }
        }
    }
}

pub fn swap_player(world: &mut World) {
    for (_, (player, speed)) in world.query::<(&mut Player, &mut Speed)>().iter() {
        match player {
            Player::Right => {
                *player = Player::Cpu;
                *speed = Speed::cpu_speed();
            }
            Player::Cpu => {
                *player = Player::Right;
                *speed = Speed::player_speed();
            }
            _ => (),
        }
    }
}

pub fn reset_paddles(world: &mut World) {
    for (_, (player, position)) in world.query::<(&Player, &mut Position)>().iter() {
        match player {
            Player::Left => *position = Position::new(20., FHEIGHT / 2.),
            Player::Right | Player::Cpu => *position = Position::new(FWIDTH - 20., FHEIGHT / 2.),
        }
    }
}

pub fn render_paddle(d: &mut RaylibMode2D<RaylibDrawHandle>, world: &World) {
    for (_, (_, position, collider, texture)) in world
        .query::<(&Player, &Position, &RectCollider, &Texture2D)>()
        .iter()
    {
        let source_rec = Rectangle::new(0., 0., collider.val.width, collider.val.height);
        let dest_rec = Rectangle::new(
            position.x,
            position.y,
            collider.val.width,
            collider.val.height,
        );
        let origin = Vector2::new(collider.val.width / 2., collider.val.height / 2.);

        d.draw_texture_pro(texture, source_rec, dest_rec, origin, 0., Color::WHITE);
    }
}
