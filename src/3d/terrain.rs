use macroquad::prelude::*;

use crate::mob::Mob;
use crate::{player::Player, textures::Textures};

pub const VOID_HEIGHT: f32 = -600.0;
pub const VOID_TRANSITION: f32 = 500.0;
pub const VOID_END: f32 = VOID_HEIGHT - VOID_TRANSITION;

const WIDTH: f32 = 200.0;

const WALL_SIZE: f32 = WIDTH * 2.0;
const WALL_SHAKE_COEFF: f32 = 35.0;
const WALL_STRETCH_COEFF: f32 = 90.0;

const WALL_POS_LIMIT: (f32, f32) = (-5.0, 5.0);
const WALL_DIM_LIMIT: (f32, f32) = (WALL_SIZE * 0.8,  WALL_SIZE * 1.2);

pub struct Terrain {
    wall_color: Color,
    grid_colors: (Color, Color),

    wall_pos: Vec3,
    wall_dim: Vec3
}

impl Terrain {
    pub fn draw(&self, txtr: &Textures) {
        clear_background(DARKGRAY);

        draw_grid(
            // Spacing is 1, so coordinates match grid position.
            WIDTH as u32, 1.0,
            self.grid_colors.0, self.grid_colors.1
        );

        draw_cube(self.wall_pos, self.wall_dim, txtr.cat, self.wall_color);
        //draw_plane()
    }

    /// uses rand (better if properly seeded)
    pub fn update(&mut self, player: &Player) {
        self.update_grid(player);
        self.update_wall(player);
    }
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            wall_color: WHITE,
            grid_colors: (WHITE, GRAY),

            wall_pos: Vec3::ZERO,
            wall_dim: Vec3::splat(WALL_SIZE)
        }
    }
}

pub fn is_outside_bounds(pos: Vec3) -> bool {
    const BORDER: f32 = WIDTH / 2.0;

    if pos.y < 0.0 { return true }

    let pos_abs = pos.abs();

    pos_abs.x > BORDER || pos_abs.z > BORDER

}

impl Terrain {
    fn update_grid(&mut self, player: &Player) {
        self.grid_colors.0 = if player.is_sprinting {
            YELLOW
        } else {
            BLACK
        };
    }

    fn update_wall(&mut self, player: &Player) {
        let player_pos = player.pos();

        // Do nothing if the player is victorious (and can therefore fly).
        if player.is_victorious {
            self.wall_color = WHITE;
        } else if is_outside_bounds(player_pos) {
            self.wall_color = RED;
        // Dynamic Feline Stress Factor (DFSF).
        } else {
            self.wall_color = WHITE;

            // Halfway to the edge of the terrain.
            let halfway = WIDTH / 4.0;
            let dist_beyond = lateral_distance(player_pos) - halfway;

            if dist_beyond > 0.0 {
                let shift_speed = dist_beyond / halfway * get_frame_time();

                self.shake_wall(shift_speed);
                self.stretch_wall(shift_speed);
            }
        }
    }
}

impl Terrain {
    fn shake_wall(&mut self, speed: f32) {
        let coords = [
            &mut self.wall_pos.x,
            &mut self.wall_pos.y,
            &mut self.wall_pos.z
        ];

        for c in coords {
            let offset = rand_sign(speed) * WALL_SHAKE_COEFF;

            *c = (*c + offset).clamp(WALL_POS_LIMIT.0, WALL_POS_LIMIT.1);
        }
    }

    fn stretch_wall(&mut self, speed: f32) {
        let coords = [
            &mut self.wall_dim.x,
            &mut self.wall_dim.y,
            &mut self.wall_dim.z
        ];

        for c in coords {
            let offset = rand_sign(speed) * WALL_STRETCH_COEFF;

            *c = (*c + offset).clamp(WALL_DIM_LIMIT.0, WALL_DIM_LIMIT.1);
        }
    }
}

/// from origin
///
/// distance from origin to `pos` projected on whichever axis gives a larger
/// distance
fn lateral_distance(pos: Vec3) -> f32 {
    let [x, _, z] = pos.abs().to_array();

    x.max(z)
}

fn rand_sign(n: f32) -> f32 {
    if rand_true(0.5) { n } else { -n }
}

/// chance is between 0 and 1
fn rand_true(chance: f32) -> bool {
    (rand::rand() as f32 / u32::MAX as f32) < chance
}
