use crate::prelude::*;

pub struct Settings {
    pub volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { volume: 0.65 }
    }
}

pub fn apply_settings(settings: &Settings, audio: Rc<RaylibAudio>) {
    audio.set_master_volume(settings.volume);
}
