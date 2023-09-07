mod assets;
mod terrain;
mod mob;
mod mouse;

use mob::{player, frog, ball};

use mob::Mob;

use assets::{Assets, Textures, Sounds};
use terrain::Terrain;
use player::Player;
use frog::Frog;
use ball::Ball;
use mouse::Mouse;

use macroquad::prelude::*;
use macroquad::audio::play_sound_once;

use ringbuf::Rb;

use ringbuf::StaticRb as RingBuf;

const FROG_COUNT: usize = 3;
const BALLS_MAX: usize = 50;    // limits memory usage

#[macroquad::main("future gastrointestinal treedee")]
async fn main() -> Result<(), FileError> {
    let assets = Assets::load().await?;
    let mut world = World::default();
    let mut mouse = Mouse::read();

    seed_rand();
    set_cursor_grab(true);
    show_mouse(false);

    loop {
        mouse.update();

        world.draw(&assets.txtr);
        world.handle_input(&mouse, &assets.snd);
        world.update(&assets.snd);

        next_frame().await;
    }
}

struct World {
    terrain: Terrain,
    player: Player,
    frogs: [Frog; FROG_COUNT],
    balls: RingBuf<Ball, BALLS_MAX>
}

impl World {
    /// returns with cam set to default
    fn draw(&self, txtr: &Textures) {
        set_camera(self.player.cam());
        self.terrain.draw(txtr);
        self.frogs.iter().for_each(|f| f.draw(txtr));
        self.balls.iter().for_each(|b| b.draw(txtr));

        set_default_camera();
        self.player.draw_view();
    }

    fn handle_input(&mut self, mouse: &Mouse, snd: &Sounds) {
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

        if is_mouse_button_pressed(MouseButton::Left) {
            play_sound_once(snd.woosh);
            self.balls.push_overwrite(player.throw_ball());
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

    fn update(&mut self, snd: &Sounds) {
        let player = &mut self.player;
        let frogs = &self.frogs;

        player.update();

        let game_is_won = frogs[0].intersects(&frogs[1]) &&
            frogs[1].intersects(&frogs[2]) &&
            frogs.iter().all(Frog::is_outside_bounds);

        if game_is_won { player.is_victorious = true }

        for f in &mut self.frogs {
            if player.intersects(f) { player.kick(f) }
            f.update();
        }

        for b in self.balls.iter_mut() {
            for f in &mut self.frogs {
                if b.intersects(f) {
                    play_sound_once(snd.croak);
                    b.strike(f);
                }
            }

            b.update();
        }

        self.terrain.update(player);
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            terrain: Terrain::default(),
            player: Player::default(),
            frogs: [
                Frog::new(vec3(4.0,  4.0, -4.0), VIOLET),
                Frog::new(vec3(6.0,  4.0,  5.0), GREEN),
                Frog::new(vec3(-5.0, 4.0,  2.0), BLUE)
            ],
            balls: RingBuf::default()
        }
    }
}

fn seed_rand() {
    use std::process;

    rand::srand(process::id() as _);
}
