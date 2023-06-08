use Direction::*;
use ZoomKind::*;

use super::Mob;
use super::frog::Frog;

use crate::terrain;

use terrain::{VOID_HEIGHT, VOID_TRANSITION};

use macroquad::prelude::*;

// Average height in meters.
const CAM_HEIGHT: f32 = 1.69;
const CAM_POS: Vec3 = vec3(0.0, CAM_HEIGHT, 0.0);

const LOOK_SPEED: f32 = 0.02;
const ZOOM_SPEED: f32 = 2.0;

// In meters per second, ignoring physical forces.
const WALK_SPEED: f32 = 35.0;
const STRAFE_SPEED: f32 = WALK_SPEED / 2.0;
const JUMP_SPEED: f32 = 10.0;

const SPRINT_COEFF: f32 = 2.0;
const PUSH_COEFF: f32 = 0.25;

pub struct Player {
    cam: Camera3D,
    /// Rotation.
    rot: Vec3,
    vel: Vec3,

    front: Vec3,
    right: Vec3,

    pub is_sprinting: bool,
    pub is_showing_stats: bool,
    pub is_victorious: bool
}

#[derive(Clone, Copy)]
pub enum Direction {
    Front,
    Back,
    Right,
    Left
}

#[derive(Clone, Copy)]
pub enum ZoomKind {
    In,
    Out
}

impl Player {
    /// the player can move if they either
    /// - are in contact with the ground
    /// - are victorious and above the void
    pub fn can_move(&self) -> bool {
        self.is_victorious && !self.is_in_void()
        || self.is_on_ground()
    }

    pub fn cam(&self) -> &Camera3D {
        &self.cam
    }

    /// draws additional elements of player vision (stats, shroud...)
    /// requires default cam to be set: `set_default_camera()`
    pub fn draw_view(&self) {
        self.draw_void_shroud();

        if self.is_showing_stats {
            self.draw_stats();
        }

        if self.is_victorious {
            self.draw_victory_text();
        }
    }

    pub fn look(&mut self, mouse_delta: Vec2) {
        const NORTH: Vec3 = vec3(0.0, 1.0, 0.0);
        const MAX_PITCH: f32 = 1.5;     // Straight up.

        let (pitch, yaw) = (&mut self.rot.z, &mut self.rot.y);
        let time_delta = get_frame_time();

        *pitch = (*pitch - mouse_delta.y * LOOK_SPEED * time_delta)
            .clamp(-MAX_PITCH, MAX_PITCH);      // Don't break your neck!

        *yaw += mouse_delta.x * LOOK_SPEED * time_delta;

        let pitch_cos = pitch.cos();

        self.front = vec3(
            yaw.cos() * pitch_cos,
            pitch.sin(),
            yaw.sin() * pitch_cos
        ).normalize();

        self.right = self.front.cross(NORTH).normalize();
        self.cam.up = self.right.cross(self.front).normalize();
        self.cam.target = self.cam.position + self.front;
    }

    pub fn zoom(&mut self, kind: ZoomKind) {
        // The default FOV is 45.3 degrees.
        const FOV_MIN: f32 = 44.27;
        const FOV_MAX: f32 = 46.03;

        let time_delta = get_frame_time();

        self.cam.fovy = match kind {
            In => FOV_MIN.max(self.cam.fovy - ZOOM_SPEED * time_delta),
            Out => FOV_MAX.min(self.cam.fovy + ZOOM_SPEED * time_delta)
        }
    }

    pub fn accel(&mut self, dir: Direction) {
        // Directions relative to the horizontal plane (the ground).
        let front_h = vec3(self.front.x, 0.0, self.front.z).normalize();
        let right_h = vec3(self.right.x, 0.0, self.right.z).normalize();

        let coeff = if self.is_sprinting {
            SPRINT_COEFF
        } else {
            1.0
        } * get_frame_time();

        match dir {
            Front => self.vel += front_h * coeff * WALK_SPEED,
            Back  => self.vel -= front_h * coeff * STRAFE_SPEED,
            Right => self.vel += right_h * coeff * STRAFE_SPEED,
            Left  => self.vel -= right_h * coeff * STRAFE_SPEED
        }
    }

    /// for testing
    /// respects sprint
    pub fn super_leap(&mut self) {
        for _ in 0..100 { self.accel(Front) }
        for _ in 0..3   { self.jump() }
    }

    pub fn jump(&mut self) {
        self.vel.y += JUMP_SPEED;
    }

    /// can also push upwards by jumping at the same time
    pub fn kick(&self, frog: &mut Frog) {
        frog.set_vel(frog.vel() + self.vel * PUSH_COEFF);
    }

    /// this is the only function that actually moves the player
    pub fn update(&mut self) {
        self.apply_forces();
        self.apply_vel();
    }
}

impl Default for Player {
    fn default() -> Self {
        let cam = Camera3D {
            position: vec3(0.0, CAM_HEIGHT, 0.0),
            up: Vec3::Y,
            target: vec3(1.0, 1.0, 0.0),    // Position + front.
            fovy: 45.3,
            ..Default::default()
        };

        Self {
            cam, rot: Vec3::ZERO, vel: Vec3::ZERO,
            front: Vec3::X, right: Vec3::Z,
            is_sprinting: false, is_showing_stats: false, is_victorious: false
        }
    }
}

impl Player {
    fn is_in_void(&self) -> bool {
        self.pos().y <= terrain::VOID_END
    }
}

/// functions require default cam to be set: `set_default_camera()`
impl Player {
    /// draws progressively more intense darkness in void
    fn draw_void_shroud(&self) {
        let dist_below = VOID_HEIGHT - self.pos().y;
        let depth_ratio = dist_below / VOID_TRANSITION;

        if depth_ratio > 0.0 {
            draw_rectangle(
                0.0, 0.0, screen_width(), screen_height(),
                Color::new(0.0, 0.0, 0.0, depth_ratio)
            );
        }
    }

    fn draw_stats(&self) {
        let [x, y, z] = self.pos().to_array();
        let pos_text = format!("Position: {x:.2} / {y:.2} / {z:.2}");

        // Ignore vertical velocity.
        let vel_h = vec3(self.vel.x, 0.0, self.vel.z);
        let vel_text = format!("Velocity: {:.2} m/s", vel_h.length());

        draw_text(&pos_text, 10.0, 25.0, 30.0, WHITE);
        draw_text(&vel_text, 10.0, 60.0, 30.0, WHITE);
    }

    fn draw_victory_text(&self) {
        // From the bottom of the screen.
        let y = screen_height() - 50.0;

        let color = if self.is_in_void() {
            RED
        } else {
            YELLOW
        };

        draw_text("VICTORY", 60.0, y, 175.0, color);
    }
}

impl Mob for Player {
    fn pos(&self) -> Vec3 { self.cam.position - CAM_POS }
    fn vel(&self) -> Vec3 { self.vel }
    // Dimensions of an average human.
    fn dim(&self) -> Vec3 { vec3(0.26, CAM_HEIGHT, 0.41) }

    fn set_pos(&mut self, val: Vec3) {
        self.cam.position = val + CAM_POS;
        self.cam.target = self.cam.position + self.front;
    }

    fn set_vel(&mut self, val: Vec3) { self.vel = val }
}
