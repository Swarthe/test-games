use super::Mob;

use crate::Textures;

use macroquad::prelude::*;

const DIM: Vec3 = vec3(0.25, 0.25, 0.25);
const HALF: Vec3 = Vec3::splat(DIM.x / 2.0);

pub struct Ball {
    pos: Vec3,
    vel: Vec3
}

impl Ball {
    pub fn new(pos: Vec3, vel: Vec3) -> Self {
        Self { pos, vel }
    }

    pub fn draw(&self, txtr: &Textures) {
        draw_cube(self.pos + HALF, self.dim(), txtr.ball, WHITE);
    }

    pub fn update(&mut self) {
        self.apply_forces();
        self.apply_vel();
    }

    /// heavily slows down self and applies velocity to `mob`
    pub fn strike<M: Mob>(&mut self, mob: &mut M) {
        // TODO: consts
        mob.set_vel(mob.vel() + self.vel / 4.0);
        self.vel /= 10.0;
    }
}

impl Mob for Ball {
    fn pos(&self) -> Vec3 { self.pos }
    fn vel(&self) -> Vec3 { self.vel }
    // TODO: average dimensions of small ball
    fn dim(&self) -> Vec3 { DIM }

    fn set_pos(&mut self, val: Vec3) { self.pos = val }
    fn set_vel(&mut self, val: Vec3) { self.vel = val }
}
