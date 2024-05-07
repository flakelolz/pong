use crate::prelude::*;

pub struct Ball;

pub fn spawn_ball(rl: &mut RaylibHandle, thread: &RaylibThread, world: &mut World) -> Entity {
    let mut image =
        Image::load_image_from_mem(".png", get_file("textures/ball.png").unwrap()).unwrap();

    let (ball_w, ball_h) = (32., 32.);

    image.resize(ball_w as i32, ball_h as i32);

    let texture = rl.load_texture_from_image(thread, &image).unwrap();

    world.spawn((
        Ball,
        Position::new(FWIDTH / 2., FHEIGHT / 2.),
        Speed::new(500., 500.),
        CircCollider::new(ball_w, ball_h),
        false,
        texture,
    ))
}

pub fn move_ball(rl: &mut RaylibHandle, world: &mut World, sounds: &[Sound]) {
    for (_, (_, pos, collider, speed)) in world
        .query::<(&Ball, &mut Position, &CircCollider, &mut Speed)>()
        .iter()
    {
        pos.x += speed.x * rl.get_frame_time();
        pos.y += speed.y * rl.get_frame_time();

        if pos.y + collider.val.y / 2. >= FHEIGHT || pos.y - collider.val.y / 2. <= 0. {
            speed.y *= -1.;

            let rng: i32 = rl.get_random_value(0..4);
            sounds[rng as usize].play();
        }

        if pos.x + collider.val.x / 2. >= FWIDTH || pos.x - collider.val.x / 2. <= 0. {
            speed.x *= -1.;
            let rng: i32 = rl.get_random_value(0..4);
            sounds[rng as usize].play();
        }
    }
}

pub fn ball_collision(world: &mut World, sound: &Sound) {
    if let Some((_, (_, b_pos, b_collider, b_speed, has_collided))) = world
        .query::<(&Ball, &Position, &CircCollider, &mut Speed, &mut bool)>()
        .iter()
        .next()
    {
        for (_, (_, p_collider)) in world.query::<(&Player, &RectCollider)>().iter() {
            if p_collider
                .val
                .check_collision_circle_rec(b_pos, b_collider.radius)
                && !*has_collided
            {
                *has_collided = true;
                b_speed.x *= -1.;
                sound.play();
            }
        }

        if ((FWIDTH / 2.) - 25.0..(FWIDTH / 2.) + 25.0).contains(&b_pos.x) && *has_collided {
            *has_collided = false;
        }
    }
}

pub fn reset_ball(world: &mut World) {
    for (_, (_, pos, has_collided)) in world.query::<(&Ball, &mut Position, &mut bool)>().iter() {
        pos.x = FWIDTH / 2.;
        pos.y = FHEIGHT / 2.;
        *has_collided = false;
    }
}

pub fn render_ball(d: &mut RaylibTextureMode<RaylibDrawHandle>, world: &World) {
    for (_, (_, position, collider, texture)) in world
        .query::<(&Ball, &Position, &CircCollider, &Texture2D)>()
        .iter()
    {
        let source_rec = rrect(0., 0., collider.val.x, collider.val.y);
        let dest_rec = rrect(position.x, position.y, collider.val.x, collider.val.y);
        let origin = rvec2(collider.val.x / 2., collider.val.y / 2.);

        d.draw_texture_pro(texture, source_rec, dest_rec, origin, 0., Color::WHITE);
    }
}
