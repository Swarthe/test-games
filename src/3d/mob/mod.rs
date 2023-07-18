use crate::terrain::is_outside_bounds;

use macroquad::prelude::*;

pub mod player;
pub mod frog;
pub mod ball;

/// for mobile entities
pub trait Mob {
    fn pos(&self) -> Vec3;
    fn vel(&self) -> Vec3;
    /// coords: forward, up, right
    fn dim(&self) -> Vec3;

    fn set_pos(&mut self, val: Vec3);
    fn set_vel(&mut self, val: Vec3);

    fn is_outside_bounds(&self) -> bool {
        is_outside_bounds(self.pos())
    }

    fn is_on_ground(&self) -> bool {
        is_on_ground(self.pos())
    }

    ///// surface area in metres
    //fn area(&self) -> f32 {
    //    let dim = self.dim();

    //    2.0 * dim.x * dim.y +
    //    2.0 * dim.y * dim.z +
    //    2.0 * dim.z * dim.x
    //}

    fn intersects<M: Mob>(&self, other: &M) -> bool {
        let (min_a, min_b) = (self.pos(),         other.pos());
        let (max_a, max_b) = (self.dim() + min_a, other.dim() + min_b);

        max_a.x >= min_b.x && min_a.x <= max_b.x &&
        max_a.y >= min_b.y && min_a.y <= max_b.y &&
        max_a.z >= min_b.z && min_a.z <= max_b.z
    }

    fn apply_forces(&mut self) {
        const G: f32 = 30.0;    // Allows realistic jumping.
        const R: f32 = 60.5;    // Allows realistic terminal velocity.
        const F: f32 = 70.0;    // Allows realistic braking.

        let time_delta = get_frame_time();
        let (pos, mut vel) = (self.pos(), self.vel());
        let is_on_ground = is_on_ground(pos);

        // Gravity.
        if !is_on_ground {
            vel.y -= G * time_delta;
        }

        // Air resistance.
        vel /= R * time_delta;

        // Friction.
        if is_on_ground {
            vel /= F * time_delta;
        }

        self.set_vel(vel);
    }

    fn apply_vel(&mut self) {
        let time_delta = get_frame_time();
        let (mut pos, vel) = (self.pos(), self.vel());

        pos.x += vel.x * time_delta;
        pos.z += vel.z * time_delta;

        // Prevent falling through the ground.
        pos.y = if !is_outside_bounds(pos) {
            0.0_f32.max(pos.y + vel.y * time_delta)
        } else {
            pos.y + vel.y * time_delta
        };

        self.set_pos(pos);
    }
}

fn is_on_ground(pos: Vec3) -> bool {
    !is_outside_bounds(pos) && pos.y == 0.0
}
