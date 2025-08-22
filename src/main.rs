mod renderer;
mod scenes;
mod shape;
mod types;

use pixels::{Pixels, SurfaceTexture};
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

use crate::renderer::draw_frame;

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

    let mut draw_normals = false;
    let mut scene = 3;

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
                    // Draw our scene into the frame
                    draw_frame(pixels.frame_mut(), fb_width, fb_height, draw_normals, scene);

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
                    if state == ElementState::Pressed && !repeat {
                        match physical_key {
                            PhysicalKey::Code(KeyCode::KeyN) => {
                                draw_normals = !draw_normals;
                                window.request_redraw();
                            }
                            PhysicalKey::Code(KeyCode::KeyZ) => {
                                // Cycle scenes: 0 (default), 1 (cornell), 2 (axes), 3 (cylinder+plane)
                                scene = (scene + 1) % 4;
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
