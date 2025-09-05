mod camera;
mod renderer;
mod scenes;
mod shape;
mod types;

use camera::Camera;
use renderer::{RenderMode, Renderer};
use scenes::{make_axes_scene, make_cornell_scene, make_default_scene, make_scene_cylinder_plane};
use shape::Shape;
use std::time::Duration;
use types::Light;

use crate::renderer::RenderMode::Raycast;
use eframe::egui::{Context, Ui, Vec2};
use eframe::{Frame, egui};
use glam::{Vec3, Vec4};
use strum::IntoEnumIterator;

struct App {
    samples: u32,
    elapsed: f64,
    render_mode: RenderMode,
    scene: usize,
    last_scene: usize,
    last_mode: RenderMode,
    last_size: [usize; 2],
    renderer: Renderer,
    iterations: usize,
}

impl App {
    pub fn new() -> Self {
        let (camera, lights, shapes) = load_scene(1);
        let renderer = Renderer::new(5, 5, RenderMode::Raycast, camera, lights, shapes);

        Self {
            samples: 1,
            elapsed: 0.0,
            render_mode: RenderMode::Raycast,
            scene: 1,
            last_scene: 1,
            last_mode: Raycast,
            last_size: [5, 5],
            renderer,
            iterations: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::right("right").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.samples, 1..=100).text("Samples"));
            ui.label(format!("Using Samples: {}", self.samples));
            for mode in RenderMode::iter() {
                ui.radio_value(
                    &mut self.render_mode,
                    mode,
                    <RenderMode as Into<&'static str>>::into(mode),
                );
            }
            egui::ComboBox::from_label("Scene")
                .selected_text(format!("{}", self.scene))
                .show_ui(ui, |ui| {
                    for i in 0..10 {
                        ui.selectable_value(&mut self.scene, i, format!("{}", i));
                    }
                });
            ui.label(format!("Rendering Time: {:.5}s", self.elapsed));
            ui.label(format!("Iterations: {:}", self.iterations));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = get_max_quadratic_size(ui);
            if (size.x as usize != self.last_size[0]
                || size.y as usize != self.last_size[1]
                || self.render_mode != self.last_mode
                || self.scene != self.last_scene)
            {
                let (camera, lights, shapes) = load_scene(self.scene as u8);
                self.last_size = [size.x as usize, size.y as usize];
                self.last_scene = self.scene;
                self.last_mode = self.render_mode;
                self.renderer = Renderer::new(
                    size.x as usize,
                    size.y as usize,
                    self.render_mode,
                    camera,
                    lights,
                    shapes,
                );
                self.iterations = 0;
            }
            let start_time = std::time::Instant::now();
            let mut pixels = render(
                [size.x as usize, size.y as usize],
                self.samples,
                &mut self.renderer,
            );

            ui.add(
                egui::Image::new(egui::load::SizedTexture::new(
                    &ctx.load_texture(
                        "raytraced",
                        egui::ColorImage::from_rgba_unmultiplied(
                            [size.x as usize, size.y as usize],
                            pixels.as_mut_slice(),
                        ),
                        Default::default(),
                    ),
                    size,
                ))
                .fit_to_exact_size(size),
            );
            self.iterations += 1;
            self.elapsed = start_time.elapsed().as_secs_f64();
            ctx.request_repaint_after(Duration::from_millis(10))
        });
    }
}

pub fn get_max_quadratic_size(ui: &mut Ui) -> Vec2 {
    let size = ui.available_size();
    let min_size = if size.x < size.y { size.x } else { size.y };
    let size = Vec2::from([min_size, min_size]);
    size
}

fn load_scene(scene: u8) -> (Camera, Vec<Light>, Vec<Shape>) {
    match scene {
        1 => make_cornell_scene(),
        2 => make_axes_scene(),
        3 => make_scene_cylinder_plane(),
        _ => make_default_scene(),
    }
}

fn render(size: [usize; 2], samples: u32, renderer: &mut Renderer) -> Vec<u8> {
    let frame_buffer = renderer.render();

    let inv_gamma = 1.0 / 2.2;
    let exposure_inv = compute_exposure_inv(&frame_buffer);

    frame_buffer
        .iter()
        .map(|x| {
            (x / (exposure_inv + x))
                .clamp(Vec3::ZERO, Vec3::ONE)
                .powf(inv_gamma)
        })
        .flat_map(|v3| v3.extend(1.0).to_array())
        .map(|v4| (v4 * 255.0) as u8)
        .collect::<Vec<u8>>()
}

fn compute_exposure_inv(frame_buffer: &Vec<Vec3>) -> f32 {
    let l = Vec3::new(0.2126, 0.7152, 0.0722);
    let sum_log = frame_buffer
        .iter()
        .map(|x| x.dot(l))
        .map(|x| (x + 1.0 / 512.0).ln())
        .sum::<f32>();

    let luminance = (sum_log / (frame_buffer.len() as f32)).exp();
    let exposure_inv = luminance / 0.18;
    exposure_inv
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions::default();

    eframe::run_native("Rrrrrrr", options, Box::new(|_cc| Ok(Box::new(App::new()))))
}
