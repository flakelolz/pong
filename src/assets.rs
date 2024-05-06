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
            let wave = audio
                .new_wave_from_memory(
                    ".ogg",
                    get_file(format!("sounds/score_{}.ogg", i).as_str()).unwrap(),
                )
                .unwrap();

            let sound = audio.new_sound_from_wave(&wave).unwrap();
            scores.push(sound);
        }

        let mut walls = Vec::new();
        for i in 0..5 {
            let wave = audio
                .new_wave_from_memory(
                    ".ogg",
                    get_file(format!("sounds/wall_{}.ogg", i).as_str()).unwrap(),
                )
                .unwrap();

            let sound = audio.new_sound_from_wave(&wave).unwrap();
            walls.push(sound);
        }

        let wave = audio
            .new_wave_from_memory(".ogg", get_file("sounds/bounce.ogg").unwrap())
            .unwrap();
        let bounce = audio.new_sound_from_wave(&wave).unwrap();

        Self {
            bounce,
            scores,
            walls,
        }
    }
}

pub fn get_file(path: &str) -> Option<&'static [u8]> {
    Some(ASSETS.get_file(path)?.contents())
}
