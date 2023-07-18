use macroquad::prelude::*;

pub struct Textures {
    pub cat: Texture2D,
    pub frog: Texture2D,
    pub ball: Texture2D
}

impl Textures {
    pub async fn load() -> Result<Self, FileError> {
        Ok(Self {
            cat: load_texture("assets/cat.png").await?,
            frog: load_texture("assets/frog.png").await?,
            // TODO: should probably be a shaded ball
            ball: Texture2D::from_rgba8(1, 1, &[255, 0, 0, 255])
        })
    }
}
