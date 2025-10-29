// Lifted from https://github.com/glium/glium/blob/master/examples/support/camera.rs
// With a bunch of changes to make it work for my program

use std::f32::consts::PI;

use glium::winit::{keyboard::{KeyCode, PhysicalKey}};

use crate::{terrain::{TERRAIN_CELL_WIDTH, TERRAIN_GRID_ROWS, WORLD_WIDTH}, world::World};
use crate::render::mat4_def::Mat4;
use crate::render::vector_math::*;

const WORLD_UP: (f32, f32, f32) = (0.0, 1.0, 0.0);

const WALKING_MOVEMENT_SPEED: f32 = 40.0;
const FLYING_MOVEMENT_SPEED: f32 = 100.0;

const PLAYER_HEIGHT: f32 = 10.0;

pub struct CameraState {
    aspect_ratio: f32,
    pub position: (f32, f32, f32),
    direction: (f32, f32, f32),

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    is_flying: bool
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
            is_flying: false
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_perspective(&self) -> Mat4 {
        return Mat4::perspective(PI / 2.0, self.aspect_ratio, 0.1, 1024.0);
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = normalize(self.direction);

        let right = normalize(cross(WORLD_UP, f));
        let camera_up = cross(f, right);

        let s = cross(f, camera_up);

        let s_norm = normalize(s);

        let u = cross(s_norm, f);

        let p = (
            -dot(self.position, s),
            -dot(self.position, u),
            -dot(self.position, f),
        );

        // note: remember that this is column-major, so the lines of code are actually columns
        return [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0     , p.1, p.2, 1.0],
        ];
    }

    pub fn update(&mut self, delta_time: f32, world: &World) {
        let right = normalize(cross(WORLD_UP, self.direction));
        let camera_forward_flat = normalize((self.direction.0, 0.0, self.direction.2));


        let movement_speed: f32 = if self.is_flying { FLYING_MOVEMENT_SPEED } else { WALKING_MOVEMENT_SPEED };

        if self.moving_up {
            self.position.1 += delta_time * movement_speed;
        }

        if self.moving_left {
            self.position.0 += right.0 * delta_time * movement_speed;
            self.position.2 += right.2 * delta_time * movement_speed;
        }

        if self.moving_down {
            self.position.1 -= delta_time * movement_speed;
        }

        if self.moving_right {
            self.position.0 -= right.0 * delta_time * movement_speed;
            self.position.2 -= right.2 * delta_time * movement_speed;
        }

        if self.moving_forward {
            self.position.0 += camera_forward_flat.0 * delta_time * movement_speed;
            self.position.2 += camera_forward_flat.2 * delta_time * movement_speed;
        }

        if self.moving_backward {
            self.position.0 -= camera_forward_flat.0 * delta_time * movement_speed;
            self.position.2 -= camera_forward_flat.2 * delta_time * movement_speed;
        }

        self.position.0 = self.position.0.rem_euclid(WORLD_WIDTH);
        self.position.2 = self.position.2.rem_euclid(WORLD_WIDTH);

        let height_as_pos = world.terrain.get_height(self.position.0 / TERRAIN_CELL_WIDTH, self.position.2 / TERRAIN_CELL_WIDTH);
        if self.is_flying && self.position.1 < height_as_pos + PLAYER_HEIGHT {
            self.is_flying = false;
        }
        if !self.is_flying {
            self.position.1 = height_as_pos + PLAYER_HEIGHT;
        }
    }

    pub fn process_input(&mut self, event: &glium::winit::event::KeyEvent) {
        let pressed = event.state == glium::winit::event::ElementState::Pressed;
        match &event.physical_key {
            PhysicalKey::Code(KeyCode::Space) => {self.moving_up = pressed; self.is_flying = true;},
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