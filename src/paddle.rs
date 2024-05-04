use crate::prelude::*;

pub struct Paddle;

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
    let image = Image::load_image("assets/paddle.png").unwrap();

    let (paddle_w, paddle_h) = (24., 96.);
    let (speed_x, speed_y) = (7., 7.);

    match player {
        Player::Left => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            world.spawn((
                Paddle,
                Player::Left,
                Position::new(20., WHEIGHT as f32 / 2.),
                Collider::new(paddle_w, paddle_h),
                Speed::new(speed_x, speed_y),
                texture,
            ))
        }
        Player::Right => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            world.spawn((
                Paddle,
                Player::Right,
                Position::new(WWIDTH as f32 - 20., WHEIGHT as f32 / 2.),
                Collider::new(paddle_w, paddle_h),
                Speed::new(speed_x, speed_y),
                texture,
            ))
        }
        Player::Cpu => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            world.spawn((
                Paddle,
                Player::Cpu,
                Position::new(WWIDTH as f32 - 20., WHEIGHT as f32 / 2.),
                Collider::new(paddle_w, paddle_h),
                Speed::new(6., 6.),
                texture,
            ))
        }
    }
}

pub fn move_paddle(rl: &RaylibHandle, world: &mut World) {
    for (_, (_, player, pos, collider, speed)) in world
        .query::<(&Paddle, &Player, &mut Position, &Collider, &Speed)>()
        .iter()
    {
        // Limit movement
        if pos.y - collider.y / 2. <= 0. {
            pos.y = collider.y / 2.;
        }
        if pos.y + collider.y / 2. >= WHEIGHT as f32 {
            pos.y = WHEIGHT as f32 - collider.y / 2.;
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
    }
}

pub fn render_paddle(d: &mut RaylibMode2D<RaylibDrawHandle>, world: &World) {
    for (_, (_, position, collider, texture)) in world
        .query::<(&Paddle, &Position, &Collider, &Texture2D)>()
        .iter()
    {
        let source_rec = Rectangle::new(0., 0., collider.x, collider.y);
        let dest_rec = Rectangle::new(position.x, position.y, collider.x, collider.y);
        let origin = Vector2::new(collider.x / 2., collider.y / 2.);

        d.draw_texture_pro(texture, source_rec, dest_rec, origin, 0., Color::WHITE);
    }
}
