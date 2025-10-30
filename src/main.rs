#![allow(dead_code)]

#[macro_use]
extern crate glium;
use glium::Surface;

use std::time::Instant;
use std::fs;

extern crate rand;
extern crate noise;

mod plant;
mod render;
mod terrain;
mod world;

use crate::plant::genome::PlantGenome;
use crate::world::World;
use crate::render::camera::CameraState;
use crate::terrain::TERRAIN_CELL_WIDTH;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Plant Evolution Simulator")
        .build(&event_loop);

    window.set_maximized(true);
    window.set_cursor_grab(glium::winit::window::CursorGrabMode::Confined).ok();
    window.set_cursor_visible(false);

    let vertex_shader_src = fs::read_to_string("src/render/shader/vertex_shader.glsl").expect("Unable to read vertex shader source");
    let vertex_shader_src = vertex_shader_src.as_str();
    let fragment_shader_src = fs::read_to_string("src/render/shader/fragment_shader.glsl").expect("Unable to read fragment shader source");
    let fragment_shader_src = fragment_shader_src.as_str();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let start_time = Instant::now();
    let mut total_time: f64 = 0.0;
    let mut delta_time: f64 = 0.0;
    let mut prev_instant = Instant::now();
    let mut total_ticks: u64 = 0;

    let mut world: World = World::world_init();

    let mut camera: CameraState = CameraState::new();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
        line_width: Some(4.0),
        
        .. Default::default()
    };

    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let instant_now = Instant::now();
                    total_time = instant_now.duration_since(start_time).as_secs_f64();
                    delta_time = instant_now.duration_since(prev_instant).as_secs_f64();
                    prev_instant = instant_now;

                    camera.update(delta_time as f32, &world);

                    let mut target = display.draw();
                    target.clear_color_and_depth((0.60, 0.75, 0.95, 1.0), 1.0);

                    // 20 ticks per second
                    while total_ticks < (total_time * 20.0) as u64 {
                        total_ticks += 1;
                        world.tick(total_ticks, &display, &camera);
                    }

                    world.render(&mut target, &program, &display, &camera, &params);

                    target.finish().unwrap();
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                    match event.physical_key {
                        glium::winit::keyboard::PhysicalKey::Code(glium::winit::keyboard::KeyCode::Escape) => {
                            window_target.exit();
                        },
                        glium::winit::keyboard::PhysicalKey::Code(glium::winit::keyboard::KeyCode::KeyP) => {
                            if event.state.is_pressed() {
                                println!("Number of concurrent Plants: {}", world.plants.internal_vec.len());
                            }
                        },
                        glium::winit::keyboard::PhysicalKey::Code(glium::winit::keyboard::KeyCode::KeyK) => {
                            if event.state.is_pressed() {
                                let new_plant = plant::Plant::new(
                                    PlantGenome::human_made_tree_genome(), 
                                    camera.position.0 / TERRAIN_CELL_WIDTH, 
                                    camera.position.2 / TERRAIN_CELL_WIDTH, 
                                    1000.0, 
                                    &world.terrain
                                );
                                world.plants.add_plant(new_plant);
                            }
                        },glium::winit::keyboard::PhysicalKey::Code(glium::winit::keyboard::KeyCode::KeyR) => {
                            if event.state.is_pressed() {
                                for _i in 0..100 {
                                    let (x, z) = (rand::random_range(0.0 .. terrain::TERRAIN_GRID_ROWS as f32), rand::random_range(0.0 .. terrain::TERRAIN_GRID_ROWS as f32));

                                    if world.terrain.get_height(x, z) < world.terrain.water_height {
                                        continue;
                                    }

                                    let new_plant = plant::Plant::new(
                                        PlantGenome::human_made_tree_genome(), 
                                        x,
                                        z,
                                        1000.0, 
                                        &world.terrain
                                    );
                                    world.plants.add_plant(new_plant);
                                }
                            }
                        },
                        _ => {
                            camera.process_input(&event);
                        }
                    }
                },
                _ => (),
            },
            glium::winit::event::Event::DeviceEvent { event, .. } => match event {
                glium::winit::event::DeviceEvent::MouseMotion { delta } => {
                    if window.has_focus() {
                        camera.process_mouse_move(delta);
                    }
                },
                _ => (),
            }
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();
}
