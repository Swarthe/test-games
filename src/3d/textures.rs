use macroquad::prelude::*;

pub struct Textures {
    pub cat: Texture2D,
    pub frog: Texture2D
}

impl Textures {
    pub async fn load() -> Result<Self, FileError> {
        Ok(Self {
            cat: load_texture("assets/cat.png").await?,
            frog: load_texture("assets/frog.png").await?
        })
    }
}
