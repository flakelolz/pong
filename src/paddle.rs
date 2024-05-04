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
            let (paddle_x, paddle_y) = (20., WHEIGHT as f32 / 2.);
            world.spawn((
                Paddle,
                Player::Left,
                Position::new(paddle_x, paddle_y),
                Rectangle::new(
                    paddle_x - paddle_w / 2.,
                    paddle_y - paddle_h / 2.,
                    paddle_w,
                    paddle_h,
                ),
                Speed::new(speed_x, speed_y),
                texture,
            ))
        }
        Player::Right => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            let (paddle_x, paddle_y) = (WWIDTH as f32 - 20., WHEIGHT as f32 / 2.);
            world.spawn((
                Paddle,
                Player::Right,
                Position::new(paddle_x, paddle_y),
                Rectangle::new(
                    paddle_x - paddle_w / 2.,
                    paddle_y - paddle_h / 2.,
                    paddle_w,
                    paddle_h,
                ),
                Speed::new(speed_x, speed_y),
                texture,
            ))
        }
        Player::Cpu => {
            let texture = rl.load_texture_from_image(thread, &image).unwrap();
            let (paddle_x, paddle_y) = (WWIDTH as f32 - 20., WHEIGHT as f32 / 2.);
            world.spawn((
                Paddle,
                Player::Cpu,
                Position::new(paddle_x, paddle_y),
                Rectangle::new(
                    paddle_x - paddle_w / 2.,
                    paddle_y - paddle_h / 2.,
                    paddle_w,
                    paddle_h,
                ),
                Speed::new(5., 5.),
                texture,
            ))
        }
    }
}

pub fn move_paddle(rl: &RaylibHandle, world: &mut World) {
    for (_, (_, player, pos, collider, speed)) in world
        .query::<(&Paddle, &Player, &mut Position, &mut Rectangle, &Speed)>()
        .iter()
    {
        // Limit movement
        if pos.y - collider.height / 2. <= 0. {
            pos.y = collider.height / 2.;
        }
        if pos.y + collider.height / 2. >= WHEIGHT as f32 {
            pos.y = WHEIGHT as f32 - collider.height / 2.;
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
        collider.x = pos.x - collider.width / 2.;
        collider.y = pos.y - collider.height / 2.;
    }
}

pub fn render_paddle(d: &mut RaylibMode2D<RaylibDrawHandle>, world: &World) {
    for (_, (_, position, collider, texture)) in world
        .query::<(&Paddle, &Position, &Rectangle, &Texture2D)>()
        .iter()
    {
        let source_rec = Rectangle::new(0., 0., collider.width, collider.height);
        let dest_rec = Rectangle::new(position.x, position.y, collider.width, collider.height);
        let origin = Vector2::new(collider.width / 2., collider.height / 2.);

        d.draw_texture_pro(texture, source_rec, dest_rec, origin, 0., Color::WHITE);
        // d.draw_rectangle_rec(collider, Color::MEDIUMSPRINGGREEN);
        // d.draw_circle(position.x as i32, position.y as i32, 1., Color::RED);
    }
}
