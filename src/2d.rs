use Direction::*;

use macroquad::prelude::*;

const TILE_NUM: (i32, i32) = (11, 7);
const TILE_SIZE: f32 = (TILE_NUM.0 * 10) as f32;

const DELIM_NUM: (i32, i32) = (TILE_NUM.0 - 1, TILE_NUM.1 - 1);
const DELIM_WIDTH: f32 = TILE_SIZE / 20.0;

#[macroquad::main(conf)]
async fn main() {
    let mut grid = Grid::default();
    let mut pawn = Pawn::default();

    loop {
        clear_background(grid.bg_color);

        grid.draw();
        pawn.draw();

        if let Some(key) = get_last_key_pressed() {
            // Pawn movement.
            match key {
                KeyCode::W => pawn.step(Up),
                KeyCode::S => pawn.step(Down),
                KeyCode::D => pawn.step(Right),
                KeyCode::A => pawn.step(Left),
                _ => ()
            }
        }

        if is_key_down(KeyCode::Space) {
            pawn.invert_colors();
        }

        if rand_true(0.01) {
            grid.invert_colors();
        }

        next_frame().await;
    }
}

struct Grid {
    bg_color: Color,
    tile_color: Color,
    tile_color_alt: Color,
}

struct Pawn {
    pos: (f32, f32),
    color: Color,
    border_color: Color
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Grid {
    const SIZE: Vec2 = vec2(
        TILE_NUM.0 as f32 * TILE_SIZE + DELIM_NUM.0 as f32 * DELIM_WIDTH,
        TILE_NUM.1 as f32 * TILE_SIZE + DELIM_NUM.1 as f32 * DELIM_WIDTH
    );

    fn draw(&self) {
        let (width, height) = (TILE_SIZE, TILE_SIZE);

        for i in 0..(TILE_NUM.0) {
            for j in 0..(TILE_NUM.1) {
                // Alternate tile colors.
                let color = if (i % 2 == 0) != (j % 2 == 0) {
                    self.tile_color
                } else {
                    self.tile_color_alt
                };

                let (x, y) = (
                    i as f32 * TILE_SIZE + i as f32 * DELIM_WIDTH - 1.0,
                    j as f32 * TILE_SIZE + j as f32 * DELIM_WIDTH - 1.0
                );

                draw_rectangle(x, y, width, height, color);
            }
        }
    }

    fn invert_colors(&mut self) {
        // Pawn colors are constant for now.
        (
            self.tile_color,
            self.tile_color_alt
        ) = (
            self.tile_color_alt,
            self.tile_color
        );
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            bg_color: GRAY,
            tile_color: ORANGE,
            tile_color_alt: DARKBROWN
        }
    }
}

impl Pawn {
    fn draw(&self) {
        const RADIUS: f32 = (TILE_SIZE * 7.0) / 20.0;
        const BORDER_WIDTH: f32 = RADIUS / 10.0;

        let (x, y) = (self.pos.0, self.pos.1);

        draw_circle(x, y, RADIUS, self.color);
        draw_circle_lines(x, y, RADIUS, BORDER_WIDTH, self.border_color);
    }

    fn invert_colors(&mut self) {
        (self.color, self.border_color) = (self.border_color, self.color);
    }

    fn step(&mut self, dir: Direction) {
        const OFFSET: f32 = TILE_SIZE + DELIM_WIDTH;

        let (mut x, mut y) = self.pos;

        match dir {
            Up    => y -= OFFSET,
            Down  => y += OFFSET,
            Right => x += OFFSET,
            Left  => x -= OFFSET
        }

        // Wrap the pawn around if it passes an edge.

        if x < 0.0  {
            x = Grid::SIZE.x - TILE_SIZE / 2.0;
        } else if x > Grid::SIZE.x {
            x = TILE_SIZE / 2.0;
        }

        if y < 0.0  {
            y = Grid::SIZE.y - TILE_SIZE / 2.0;
        } else if y > Grid::SIZE.y {
            y = TILE_SIZE / 2.0;
        }

        self.pos = (x, y);
    }
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            pos: (Grid::SIZE.x / 2.0, Grid::SIZE.y / 2.0),
            color: SKYBLUE,
            border_color: RED
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "future gastrointestinal".to_string(),
        window_width: Grid::SIZE.x as i32,
        window_height: Grid::SIZE.y as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn rand_true(chance: f32) -> bool {
    // We don't seed the PRNG for now.
    (rand::rand() as f32) < (u32::MAX as f32 * chance)
}

//fn rand_color() -> Color {
//    let mut rgb = [0_u8; 3];
//
//    for sub in &mut rgb {
//        *sub = (rand::rand() % u8::MAX as u32) as u8
//    }
//
//    Color::from_rgba(rgb[0], rgb[1], rgb[2], u8::MAX)
//}
