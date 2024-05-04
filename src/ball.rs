use crate::prelude::*;

pub struct Ball;

pub fn spawn_ball(rl: &mut RaylibHandle, thread: &RaylibThread, world: &mut World) -> Entity {
    let mut image = Image::load_image("assets/ball.png").unwrap();

    let (ball_w, ball_h) = (32., 32.);

    image.resize(ball_w as i32, ball_h as i32);

    let texture = rl.load_texture_from_image(thread, &image).unwrap();

    world.spawn((
        Ball,
        Position::new(WWIDTH as f32 / 2., WHEIGHT as f32 / 2.),
        Speed::new(7., 7.),
        Vector2::new(ball_w, ball_h),
        texture,
    ))
}

pub fn move_ball(world: &mut World) {
    for (_, (_, pos, collider, speed)) in world
        .query::<(&Ball, &mut Position, &Vector2, &mut Speed)>()
        .iter()
    {
        pos.x += speed.x;
        pos.y += speed.y;

        if pos.y + collider.y / 2. >= WHEIGHT as f32 || pos.y - collider.y / 2. <= 0. {
            speed.y *= -1.;
        }

        if pos.x + collider.x / 2. >= WWIDTH as f32 || pos.x - collider.x / 2. <= 0. {
            speed.x *= -1.;
        }
    }
}

pub fn check_collision(world: &mut World) {
    if let Some((_, (_, b_pos, b_collider, b_speed))) = world
        .query::<(&Ball, &Position, &Vector2, &mut Speed)>()
        .iter()
        .next()
    {
        for (_, (_, p_collider)) in world.query::<(&Paddle, &Rectangle)>().iter() {
            if p_collider.check_collision_circle_rec(b_pos, b_collider.x / 2.) {
                b_speed.x *= -1.;
                // FIX: Fix bounce angle
            }
        }
    }
}

pub fn reset_ball(pos: &mut Position) {
    pos.x = WWIDTH as f32 / 2.;
    pos.y = WHEIGHT as f32 / 2.;
}

pub fn render_ball(d: &mut RaylibMode2D<RaylibDrawHandle>, world: &World) {
    for (_, (_, position, collider, texture)) in world
        .query::<(&Ball, &Position, &Vector2, &Texture2D)>()
        .iter()
    {
        let source_rec = Rectangle::new(0., 0., collider.x, collider.y);
        let dest_rec = Rectangle::new(position.x, position.y, collider.x, collider.y);
        let origin = Vector2::new(collider.x / 2., collider.y / 2.);

        d.draw_texture_pro(texture, source_rec, dest_rec, origin, 0., Color::WHITE);
        // d.draw_circle_v(position, 1., Color::RED);
    }
}
