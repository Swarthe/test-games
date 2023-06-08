use super::Mob;

use crate::Textures;

use macroquad::prelude::*;

const SIZE: f32 = 1.0;
const HALF: Vec3 = Vec3::splat(SIZE / 2.0);

pub struct Frog {
    pos: Vec3,
    vel: Vec3,
    color: Color
}

impl Frog {
    pub fn new(pos: Vec3, color: Color) -> Self {
        Self { pos, vel: Vec3::ZERO, color }
    }

    pub fn draw(&self, txtr: &Textures) {
        draw_cube(self.pos + HALF, self.dim(), txtr.frog, self.color);
    }

    pub fn update(&mut self) {
        self.apply_forces();
        self.apply_vel();
    }
}

impl Mob for Frog {
    fn pos(&self) -> Vec3 { self.pos }
    fn vel(&self) -> Vec3 { self.vel }
    fn dim(&self) -> Vec3 { Vec3::splat(SIZE) }

    fn set_pos(&mut self, val: Vec3) { self.pos = val }
    fn set_vel(&mut self, val: Vec3) { self.vel = val }
}
