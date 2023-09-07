use macroquad::prelude::*;

use macroquad::audio::Sound;
use macroquad::audio::load_sound;

pub struct Assets {
    pub txtr: Textures,
    pub snd: Sounds
}

pub struct Sounds {
    pub woosh: Sound,
    pub croak: Sound
}

pub struct Textures {
    pub cat: Texture2D,
    pub frog: Texture2D,
    pub ball: Texture2D
}

impl Assets {
    pub async fn load() -> Result<Self, FileError> {
        Ok(Self {
            txtr: Textures {
                cat: load_texture("assets/cat.png").await?,
                frog: load_texture("assets/frog.png").await?,
                // TODO: should probably be a shaded ball
                ball: Texture2D::from_rgba8(1, 1, &[255, 0, 0, 255])
            },
            snd: Sounds {
                woosh: load_sound("assets/woosh.wav").await?,
                croak: load_sound("assets/croak.ogg").await?,
            }
        })
    }
}
