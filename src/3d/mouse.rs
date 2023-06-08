use macroquad::prelude::*;

pub struct Mouse {
    pos: Vec2,
    /// The difference between the current and last position.
    pos_delta: Vec2
}


impl Mouse {
    pub fn read() -> Self {
        Self {
            pos: Vec2::from(mouse_position()),
            pos_delta: Vec2::ZERO,
        }
    }

    pub fn pos_delta(&self) -> Vec2 {
        self.pos_delta
    }

    pub fn has_moved(&self) -> bool {
        self.pos_delta != Vec2::ZERO
    }

    pub fn update(&mut self) {
        let new_pos = Vec2::from(mouse_position());

        self.pos_delta = new_pos - self.pos;
        self.pos = new_pos;
    }
}
