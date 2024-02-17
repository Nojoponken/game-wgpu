use cgmath::{
    num_traits::{abs, Signed},
    Angle, InnerSpace, Point3, Rad, Vector3,
};
use std::{cmp::min, f32::consts::FRAC_PI_2};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta},
    keyboard::KeyCode,
};

use crate::terrain::World;

use super::Camera;
use super::Player;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct PlayerController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
    mine_block: bool,
}

impl PlayerController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            mine_block: false,
        }
    }

    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.amount_forward = amount;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.amount_backward = amount;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.amount_left = amount;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.amount_right = amount;
                true
            }
            KeyCode::Space => {
                self.amount_up = amount;
                true
            }
            KeyCode::ShiftLeft => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }

    fn modulo(a: f32, b: f32) -> f32 {
        ((a % b) + b) % b
    }
    pub fn process_click(&mut self, player: &Player, world: &mut World) {
        let mut current = player.camera.position;

        let xz_len = player.camera.pitch.cos();

        let direction = Vector3 {
            x: xz_len * player.camera.yaw.cos(),
            y: player.camera.pitch.sin(),
            z: xz_len * player.camera.yaw.sin(),
        };

        let mut dist = 0.0;
        loop {
            let x_dist;
            let y_dist;
            let z_dist;

            if direction.x.is_positive() {
                x_dist = (1.0 - Self::modulo(current.x, 1.0)) / abs(direction.x);
            } else {
                x_dist = (Self::modulo(current.x, 1.0)) / abs(direction.x);
            }
            if direction.y.is_positive() {
                y_dist = (1.0 - Self::modulo(current.y, 1.0)) / abs(direction.y);
            } else {
                y_dist = (Self::modulo(current.y, 1.0)) / abs(direction.y);
            }
            if direction.z.is_positive() {
                z_dist = (1.0 - Self::modulo(current.z, 1.0)) / abs(direction.z);
            } else {
                z_dist = (Self::modulo(current.z, 1.0)) / abs(direction.z);
            }

            let step_size = x_dist.min(y_dist).min(z_dist) + 0.00001;

            current += direction * step_size;
            dist += step_size;
            if dist > 16.0 {
                break;
            }
            if world.block_exists(current) {
                world.remove_block(current);
                break;
            }
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32) {
        // Rotate
        camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non-cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }
    pub fn update_player(&mut self, player: &mut Player, dt: f32) {
        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = player.camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        player.velocity += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        player.velocity += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = player.camera.pitch.0.sin_cos();
        let scrollward =
            Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        player.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        //player.position.y += (self.amount_up - self.amount_down) * self.speed * dt;
        if player.velocity.y == 0.0 && self.amount_up != 0.0 {
            player.velocity.y = 0.2;
        }

        // Physics
    }
}
