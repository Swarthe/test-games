mod textures;
mod terrain;
mod mob;
mod mouse;

use mob::{player, frog};

use mob::Mob;

use textures::Textures;
use terrain::Terrain;
use player::Player;
use frog::Frog;
use mouse::Mouse;

use macroquad::prelude::*;

#[macroquad::main("future gastrointestinal treedee")]
async fn main() -> Result<(), FileError> {
    let textures = Textures::load().await?;
    let mut world = World::default();
    let mut mouse = Mouse::read();

    seed_rand();
    set_cursor_grab(true);
    show_mouse(false);

    loop {
        mouse.update();

        world.draw(&textures);
        world.handle_input(&mouse);
        world.update();

        next_frame().await;
    }
}

struct World {
    terrain: Terrain,
    player: Player,
    frogs: [Frog; 3]
}

impl World {
    /// returns with cam set to default
    fn draw(&self, txtr: &Textures) {
        set_camera(self.player.cam());
        self.terrain.draw(txtr);
        self.frogs.iter().for_each(|f| f.draw(txtr));

        set_default_camera();
        self.player.draw_view();
    }

    fn handle_input(&mut self, mouse: &Mouse) {
        use player::{Direction::*, ZoomKind::*};

        let player = &mut self.player;

        if mouse.has_moved() {
            player.look(mouse.pos_delta());
        }

        if is_key_down(KeyCode::E) { player.zoom(In) }
        if is_key_down(KeyCode::Q) { player.zoom(Out) }

        if is_key_pressed(KeyCode::Tab) {
            player.is_showing_stats = !player.is_showing_stats;
        }

        if is_key_down(KeyCode::LeftShift) {
            player.is_sprinting = true;
        } else {
            player.is_sprinting = false;
        }

        // Don't walk on the air, unless you deserve it...
        if !player.can_move() { return }

        if is_key_down(KeyCode::W) { player.accel(Front) }
        if is_key_down(KeyCode::S) { player.accel(Back) }
        if is_key_down(KeyCode::D) { player.accel(Right) }
        if is_key_down(KeyCode::A) { player.accel(Left) }

        if is_key_pressed(KeyCode::Space) { player.jump() }
        // for testing
        if is_key_pressed(KeyCode::Enter) { player.super_leap() }

    }

    fn update(&mut self) {
        let player = &mut self.player;

        player.update();
        if game_is_won(&self.frogs) { player.is_victorious = true }

        for f in &mut self.frogs {
            if player.intersects(f) { player.kick(f) }
            f.update();
        }

        self.terrain.update(player);
    }
}

impl Default for World {
    fn default() -> Self {
        let frogs = [
            Frog::new(vec3(4.0,  4.0, -4.0), VIOLET),
            Frog::new(vec3(6.0,  4.0,  5.0), GREEN),
            Frog::new(vec3(-5.0, 4.0,  2.0), BLUE)
        ];

        Self {
            terrain: Terrain::default(),
            player: Player::default(),
            frogs
        }
    }
}

fn seed_rand() {
    use std::process;

    rand::srand(process::id() as _);
}

fn game_is_won(frogs: &[Frog; 3]) -> bool {
    frogs[0].intersects(&frogs[1]) &&
    frogs[1].intersects(&frogs[2]) &&
    frogs.iter().all(Frog::is_outside_bounds)
}
