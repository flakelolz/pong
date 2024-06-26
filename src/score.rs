use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Score {
    pub left: i32,
    pub right: i32,
}

pub fn spawn_score(world: &mut World) -> Entity {
    world.spawn((Score::default(),))
}

pub fn update_score(rl: &mut RaylibHandle, world: &mut World, sound: &[Sound]) {
    if let Some((_, score)) = world.query::<&mut Score>().iter().next() {
        let left = &mut score.left;
        let right = &mut score.right;

        for (_, (_, pos, collider)) in world
            .query::<(&Ball, &mut Position, &CircCollider)>()
            .iter()
        {
            if pos.x - collider.val.x / 2. <= 0. {
                *right += 1;
                let rng: i32 = rl.get_random_value(0..4);
                sound[rng as usize].play();

                pos.x = FWIDTH / 2.;
                pos.y = FHEIGHT / 2.;
            } else if pos.x + collider.val.x / 2. >= FWIDTH {
                *left += 1;
                let rng: i32 = rl.get_random_value(0..4);
                sound[rng as usize].play();

                pos.x = FWIDTH / 2.;
                pos.y = FHEIGHT / 2.;
            }
        }
    }
}

pub fn reset_score(world: &mut World) {
    if let Some((_, score)) = world.query::<&mut Score>().iter().next() {
        score.left = 0;
        score.right = 0;
    }
}

pub fn render_score(d: &mut RaylibTextureMode<RaylibDrawHandle>, world: &World) {
    if let Some((_, score)) = world.query::<&Score>().iter().next() {
        let left = &score.left;
        let right = &score.right;
        d.draw_text(
            format!("{}", left).as_str(),
            WIDTH / 4 - 20,
            20,
            50,
            Color::WHITE,
        );
        d.draw_text(
            format!("{}", right).as_str(),
            3 * WIDTH / 4 - 20,
            20,
            50,
            Color::WHITE,
        );
    }
}
