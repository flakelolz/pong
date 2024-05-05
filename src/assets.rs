use crate::prelude::*;

pub struct Assets<'a> {
    pub bounce: Sound<'a>,
    pub scores: Vec<Sound<'a>>,
    pub walls: Vec<Sound<'a>>,
}

impl<'a> Assets<'a> {
    pub fn load_assets(audio: &'a RaylibAudio) -> Self {
        let mut scores = Vec::new();
        for i in 0..5 {
            let mut sound = audio
                .new_sound(format!("assets/sounds/score_{}.ogg", i).as_str())
                .unwrap();

            sound.set_volume(0.5);
            scores.push(sound);
        }

        let mut walls = Vec::new();
        for i in 0..5 {
            let mut sound = audio
                .new_sound(format!("assets/sounds/wall_{}.ogg", i).as_str())
                .unwrap();

            sound.set_volume(0.2);
            sound.set_pitch(2.0);
            walls.push(sound);
        }

        Self {
            bounce: audio.new_sound("assets/sounds/bounce.ogg").unwrap(),
            scores,
            walls,
        }
    }
}
