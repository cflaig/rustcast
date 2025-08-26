mod camera;
mod renderer;
mod scenes;
mod shape;
mod types;

use pixels::{Pixels, SurfaceTexture};
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

use crate::camera::Camera;
use crate::renderer::{RenderMode, draw_frame};
use crate::scenes::{
    make_axes_scene, make_cornell_scene, make_default_scene, make_scene_cylinder_plane,
};
use crate::shape::Shape;
use crate::types::Light;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create event loop and window
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Blue Circle on Red Background")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 1024.0))
        .build(&event_loop)
        .unwrap();
    // Leak the window to obtain a 'static reference suitable for event loop usage
    let window: &'static winit::window::Window = Box::leak(Box::new(window));

    // Create the pixel buffer (framebuffer) tied to the window surface
    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

    // Track framebuffer size for drawing
    let mut fb_width = size.width;
    let mut fb_height = size.height;

    let mut render_mode = RenderMode::Raycast;
    let mut scene: u8 = 3;

    // Current scene data (camera/light/shapes)
    fn load_scene(scene: u8) -> (Camera, Vec<Light>, Vec<Shape>) {
        match scene {
            1 => make_cornell_scene(),
            2 => make_axes_scene(),
            3 => make_scene_cylinder_plane(),
            _ => make_default_scene(),
        }
    }
    let (mut camera, mut lights, mut shapes) = load_scene(scene);

    let mut shift_down = false;

    // Run the event loop
    Ok(event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                WindowEvent::Resized(new_size) => {
                    // Resize the surface to match the new window size
                    fb_width = new_size.width;
                    fb_height = new_size.height;
                    let _ = pixels.resize_surface(new_size.width, new_size.height);
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    // Request redraw to adapt to scaling changes
                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    // Draw our scene into the frame and measure draw time
                    let start = Instant::now();
                    draw_frame(
                        pixels.frame_mut(),
                        fb_width,
                        fb_height,
                        render_mode,
                        &camera,
                        &lights,
                        &shapes,
                    );
                    let elapsed = start.elapsed();
                    println!("Draw time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

                    // Render to the window
                    if pixels.render().is_err() {
                        // If rendering fails, exit the app
                        elwt.exit();
                    }
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state,
                            physical_key,
                            repeat,
                            ..
                        },
                    ..
                } => {
                    // Track shift state regardless of repeat
                    match physical_key {
                        PhysicalKey::Code(KeyCode::ShiftLeft)
                        | PhysicalKey::Code(KeyCode::ShiftRight) => {
                            shift_down = state == ElementState::Pressed;
                        }
                        _ => {}
                    }

                    // Handle non-repeating key presses for discrete steps
                    if state == ElementState::Pressed && !repeat {
                        let move_step = 0.2f32;
                        let rot_step = 0.05f32; // radians
                        match physical_key {
                            PhysicalKey::Code(KeyCode::KeyN) => {
                                render_mode = match render_mode {
                                    RenderMode::Raycast => RenderMode::Raytrace,
                                    RenderMode::Raytrace => RenderMode::Normals,
                                    RenderMode::Normals => RenderMode::Pathtracing,
                                    RenderMode::Pathtracing => RenderMode::Raycast,
                                };
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::KeyZ) => {
                                // Cycle scenes: 0 (default), 1 (cornell), 2 (axes), 3 (cylinder+plane)
                                scene = (scene + 1) % 4;
                                let (c, l, s) = load_scene(scene);
                                camera = c;
                                lights = l;
                                shapes = s;
                                window.request_redraw();
                            }
                            // Movement keys
                            PhysicalKey::Code(KeyCode::KeyW) => {
                                camera.move_along_up(move_step);
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::KeyS) => {
                                camera.move_along_up(-move_step);
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::KeyA) => {
                                camera.move_along_right(-move_step);
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::KeyD) => {
                                camera.move_along_right(move_step);
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::KeyR) => {
                                if shift_down {
                                    camera.move_along_look(-move_step);
                                } else {
                                    camera.move_along_look(move_step);
                                }
                                window.request_redraw();
                            }
                            // Arrow keys for yaw (left/right) and pitch (up/down)
                            PhysicalKey::Code(KeyCode::ArrowLeft) => {
                                camera.yaw(-rot_step);
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::ArrowRight) => {
                                camera.yaw(rot_step);
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::ArrowUp) => {
                                camera.pitch(rot_step);
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::ArrowDown) => {
                                camera.pitch(-rot_step);
                                window.request_redraw();
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Request a redraw at the next opportunity
                window.request_redraw();
            }
            _ => {}
        }
    })?)
}
