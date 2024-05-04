use crate::prelude::*;

pub fn update_score(world: &mut World) {
    if let Some((_, resources)) = world.query::<&mut Resources>().iter().next() {
        let left = &mut resources.score.left;
        let right = &mut resources.score.right;

        for (_, (_, pos, collider)) in world.query::<(&Ball, &mut Position, &Vector2)>().iter() {
            if pos.x - collider.x / 2. <= 0. {
                *right += 1;
                reset_ball(pos);
            } else if pos.x + collider.x / 2. >= WWIDTH as f32 {
                *left += 1;
                reset_ball(pos);
            }
        }
    }
}

pub fn render_score(d: &mut RaylibMode2D<RaylibDrawHandle>, world: &World) {
    if let Some((_, resources)) = world.query::<&Resources>().iter().next() {
        let left = &resources.score.left;
        let right = &resources.score.right;
        d.draw_text(
            format!("{}", left).as_str(),
            WWIDTH / 2 - 120,
            20,
            50,
            Color::WHITE,
        );
        d.draw_text(
            format!("{}", right).as_str(),
            WWIDTH / 2 + 100,
            20,
            50,
            Color::WHITE,
        );
    }
}
