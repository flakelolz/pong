use crate::prelude::*;

pub struct Assets<'a> {
    pub bounce: Sound<'a>,
    pub scores: Vec<Sound<'a>>,
    pub walls: Vec<Sound<'a>>,
}

impl<'a> Assets<'a> {
    pub fn load_assets(audio: &'a RaylibAudio) -> Self {
        let score_0 = include_bytes!("..\\assets\\sounds\\score_0.ogg");
        let score_1 = include_bytes!("..\\assets\\sounds\\score_1.ogg");
        let score_2 = include_bytes!("..\\assets\\sounds\\score_2.ogg");
        let score_3 = include_bytes!("..\\assets\\sounds\\score_3.ogg");
        let score_4 = include_bytes!("..\\assets\\sounds\\score_4.ogg");

        let score_0 = audio.new_wave_from_memory(".ogg", score_0).unwrap();
        let score_1 = audio.new_wave_from_memory(".ogg", score_1).unwrap();
        let score_2 = audio.new_wave_from_memory(".ogg", score_2).unwrap();
        let score_3 = audio.new_wave_from_memory(".ogg", score_3).unwrap();
        let score_4 = audio.new_wave_from_memory(".ogg", score_4).unwrap();

        let score_0 = audio.new_sound_from_wave(&score_0).unwrap();
        let score_1 = audio.new_sound_from_wave(&score_1).unwrap();
        let score_2 = audio.new_sound_from_wave(&score_2).unwrap();
        let score_3 = audio.new_sound_from_wave(&score_3).unwrap();
        let score_4 = audio.new_sound_from_wave(&score_4).unwrap();

        let scores = vec![score_0, score_1, score_2, score_3, score_4];

        let walls_0 = include_bytes!("..\\assets\\sounds\\wall_0.ogg");
        let walls_1 = include_bytes!("..\\assets\\sounds\\wall_1.ogg");
        let walls_2 = include_bytes!("..\\assets\\sounds\\wall_2.ogg");
        let walls_3 = include_bytes!("..\\assets\\sounds\\wall_3.ogg");
        let walls_4 = include_bytes!("..\\assets\\sounds\\wall_4.ogg");

        let walls_0 = audio.new_wave_from_memory(".ogg", walls_0).unwrap();
        let walls_1 = audio.new_wave_from_memory(".ogg", walls_1).unwrap();
        let walls_2 = audio.new_wave_from_memory(".ogg", walls_2).unwrap();
        let walls_3 = audio.new_wave_from_memory(".ogg", walls_3).unwrap();
        let walls_4 = audio.new_wave_from_memory(".ogg", walls_4).unwrap();

        let walls_0 = audio.new_sound_from_wave(&walls_0).unwrap();
        let walls_1 = audio.new_sound_from_wave(&walls_1).unwrap();
        let walls_2 = audio.new_sound_from_wave(&walls_2).unwrap();
        let walls_3 = audio.new_sound_from_wave(&walls_3).unwrap();
        let walls_4 = audio.new_sound_from_wave(&walls_4).unwrap();

        let walls = vec![walls_0, walls_1, walls_2, walls_3, walls_4];

        let bytes = include_bytes!("..\\assets\\sounds\\bounce.ogg");
        let wave = audio.new_wave_from_memory(".ogg", bytes).unwrap();
        let bounce = audio.new_sound_from_wave(&wave).unwrap();

        Self {
            bounce,
            scores,
            walls,
        }
    }
}
