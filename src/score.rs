use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Score {
    pub left: i32,
    pub right: i32,
}

pub fn spawn_score(world: &mut World) -> Entity {
    world.spawn((Score::default(),))
}

pub fn update_score(world: &mut World) {
    if let Some((_, score)) = world.query::<&mut Score>().iter().next() {
        let left = &mut score.left;
        let right = &mut score.right;

        for (_, (_, pos, collider)) in world
            .query::<(&Ball, &mut Position, &CircCollider)>()
            .iter()
        {
            if pos.x - collider.val.x / 2. <= 0. {
                *right += 1;
                reset_ball(pos);
            } else if pos.x + collider.val.x / 2. >= WWIDTH as f32 {
                *left += 1;
                reset_ball(pos);
            }
        }
    }
}

pub fn render_score(d: &mut RaylibMode2D<RaylibDrawHandle>, world: &World) {
    if let Some((_, score)) = world.query::<&Score>().iter().next() {
        let left = &score.left;
        let right = &score.right;
        d.draw_text(
            format!("{}", left).as_str(),
            WWIDTH / 4 - 20,
            20,
            50,
            Color::WHITE,
        );
        d.draw_text(
            format!("{}", right).as_str(),
            3 * WWIDTH / 4 - 20,
            20,
            50,
            Color::WHITE,
        );
    }
}
