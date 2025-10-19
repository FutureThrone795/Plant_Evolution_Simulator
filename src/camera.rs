// Lifted from https://github.com/glium/glium/blob/master/examples/support/camera.rs
// With a bunch of changes to make it work for my program

use glium::winit::{keyboard::{KeyCode, PhysicalKey}};

use crate::terrain::{TERRAIN_GRID_ROWS, TERRAIN_CELL_WIDTH, WORLD_WIDTH};

use crate::vector_math::*;

const WORLD_UP: (f32, f32, f32) = (0.0, 1.0, 0.0);

const MOVEMENT_SPEED: f32 = 100.0;

pub struct CameraState {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    direction: (f32, f32, f32),

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            position: (TERRAIN_CELL_WIDTH * (TERRAIN_GRID_ROWS as f32 / 2.0 - 0.5), 50.0, TERRAIN_CELL_WIDTH * (TERRAIN_GRID_ROWS as f32 / 2.0 - 0.5)),
            direction: (0.0, 0.0, 1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = normalize(self.direction);

        let right = normalize(cross(WORLD_UP, f));
        let camera_up = cross(f, right);

        let s = (f.1 * camera_up.2 - f.2 * camera_up.1,
                 f.2 * camera_up.0 - f.0 * camera_up.2,
                 f.0 * camera_up.1 - f.1 * camera_up.0);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
                 -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
                 -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0     , p.1, p.2, 1.0],
        ]
    }

    pub fn update(&mut self, delta_time: f32) {
        let right = normalize(cross(WORLD_UP, self.direction));
        let camera_forward_flat = normalize((self.direction.0, 0.0, self.direction.2));

        if self.moving_up {
            self.position.1 += delta_time * MOVEMENT_SPEED;
        }

        if self.moving_left {
            self.position.0 += right.0 * delta_time * MOVEMENT_SPEED;
            self.position.2 += right.2 * delta_time * MOVEMENT_SPEED;
        }

        if self.moving_down {
            self.position.1 -= delta_time * MOVEMENT_SPEED;
        }

        if self.moving_right {
            self.position.0 -= right.0 * delta_time * MOVEMENT_SPEED;
            self.position.2 -= right.2 * delta_time * MOVEMENT_SPEED;
        }

        if self.moving_forward {
            self.position.0 += camera_forward_flat.0 * delta_time * MOVEMENT_SPEED;
            self.position.2 += camera_forward_flat.2 * delta_time * MOVEMENT_SPEED;
        }

        if self.moving_backward {
            self.position.0 -= camera_forward_flat.0 * delta_time * MOVEMENT_SPEED;
            self.position.2 -= camera_forward_flat.2 * delta_time * MOVEMENT_SPEED;
        }

        self.position.0 = self.position.0.rem_euclid(WORLD_WIDTH);
        self.position.2 = self.position.2.rem_euclid(WORLD_WIDTH);
    }

    pub fn process_input(&mut self, event: &glium::winit::event::KeyEvent) {
        let pressed = event.state == glium::winit::event::ElementState::Pressed;
        match &event.physical_key {
            PhysicalKey::Code(KeyCode::Space) => self.moving_up = pressed,
            PhysicalKey::Code(KeyCode::ShiftLeft) => self.moving_down = pressed,
            PhysicalKey::Code(KeyCode::KeyA) => self.moving_left = pressed,
            PhysicalKey::Code(KeyCode::KeyD) => self.moving_right = pressed,
            PhysicalKey::Code(KeyCode::KeyW) => self.moving_forward = pressed,
            PhysicalKey::Code(KeyCode::KeyS) => self.moving_backward = pressed,
            _ => (),
        };
    }

    pub fn process_mouse_move(&mut self, delta: (f64, f64)) {
        let current_dir = self.direction;

        let sensitivity = 0.002;

        let yaw = -delta.0 * sensitivity;
        let pitch = delta.1 * sensitivity;

        let dir = rotate_around_axis(current_dir, WORLD_UP, yaw as f32);
        let right = normalize(cross(WORLD_UP, dir));
        let new_dir = rotate_around_axis(dir, right, pitch as f32);
        let up_dot = dot(new_dir, WORLD_UP);
        if up_dot.abs() < 0.99 {
            self.direction = normalize(new_dir);
        }
    }
}