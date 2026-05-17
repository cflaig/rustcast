use rustcast::camera::Camera;
use rustcast::renderer::{RenderMode, Renderer};
use rustcast::scenes::{
    make_axes_scene, make_box_scene, make_cornell_scene, make_default_scene,
    make_scene_cylinder_plane, make_three_spheres_scene, make_glass_and_mirror_scene,
    make_glass_sphere_scene,
};
use rustcast::shape::Shape;
use rustcast::types::Light;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use eframe::egui::{Context, Ui, Vec2};
use eframe::{Frame, egui};
use glam::Vec3;
use rustcast::renderer::RenderMode::Raycast;
use std::time::Duration;
use strum::IntoEnumIterator;
use thread_priority::{ThreadPriority, ThreadPriorityValue};

fn lower_current_thread_priority() {
    if let Err(err) =
        ThreadPriority::Crossplatform(ThreadPriorityValue::try_from(0).unwrap()).set_for_current()
    {
        eprintln!("Could not lower thread priority: {err}");
    }
}

struct App {
    samples: u32,
    elapsed: f64,
    render_mode: RenderMode,
    scene: usize,
    last_scene: usize,
    last_mode: RenderMode,
    last_size: [usize; 2],
    pixels_size: [usize; 2],
    iterations: usize,
    pixels: Vec<u8>,
    last_frame_buffer: Vec<Vec3>,
    exposure: f32,
    gamma: f32,
    start_timer: std::time::Instant,
    render_thread: Option<thread::JoinHandle<()>>,
    cancel_render: Option<Arc<AtomicBool>>,
    tx: std::sync::mpsc::Sender<(Vec<Vec3>, usize, usize)>,
    rx: std::sync::mpsc::Receiver<(Vec<Vec3>, usize, usize)>,
}

impl App {
    pub fn new() -> Self {
        let (_camera, _lights, _shapes) = load_scene(1);
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            samples: 1,
            elapsed: 0.0,
            render_mode: Raycast,
            scene: 1,
            last_scene: 1,
            last_mode: Raycast,
            last_size: [5, 5],
            pixels_size: [5, 5],
            iterations: 0,
            pixels: vec![0; 5 * 5 * 4],
            last_frame_buffer: Vec::new(),
            exposure: 0.0,
            gamma: 2.2,
            start_timer: std::time::Instant::now(),
            tx,
            rx,
            render_thread: None,
            cancel_render: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let mut params_changed = false;
        egui::SidePanel::right("right").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.samples, 1..=100).text("Samples"));
            if ui
                .add(egui::Slider::new(&mut self.exposure, -3.0..=3.0).text("Exposure"))
                .changed()
            {
                params_changed = true;
            }
            if ui
                .add(egui::Slider::new(&mut self.gamma, 1.8..=2.6).text("Gamma"))
                .changed()
            {
                params_changed = true;
            }
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
                    for i in 0..8 {
                        let name = match i {
                            1 => "Cornell Box",
                            2 => "Axes",
                            3 => "Cylinder/Plane",
                            4 => "Box Scene",
                            5 => "Three Spheres",
                            6 => "Glass & Mirror",
                            7 => "Glass Sphere",
                            _ => "Default",
                        };
                        ui.selectable_value(&mut self.scene, i, name);
                    }
                });
            ui.label(format!("Rendering Time: {:.5}s", self.elapsed));
            ui.label(format!("Iterations: {:}", self.iterations));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = get_max_quadratic_size(ui);
            if size.x as usize != self.last_size[0]
                || size.y as usize != self.last_size[1]
                || self.render_mode != self.last_mode
                || self.scene != self.last_scene
            {
                if let Some(cancel) = self.cancel_render.take() {
                    cancel.store(true, Ordering::Relaxed);
                }
                if let Some(handle) = self.render_thread.take() {
                    let _ = handle.join();
                }
                while self.rx.try_recv().is_ok() {}
                let cancel = Arc::new(AtomicBool::new(false));
                self.cancel_render = Some(Arc::clone(&cancel));

                let (camera, lights, shapes) = load_scene(self.scene as u8);
                self.last_size = [size.x as usize, size.y as usize];
                self.pixels_size = self.last_size;
                self.last_scene = self.scene;
                self.last_mode = self.render_mode;
                self.pixels = vec![0; size.x as usize * size.y as usize * 4];
                self.last_frame_buffer.clear();
                let mut renderer = Renderer::new(
                    size.x as usize,
                    size.y as usize,
                    self.render_mode,
                    camera,
                    lights,
                    shapes,
                );
                self.iterations = 0;
                let tx_channel = self.tx.clone();

                self.render_thread = Some(thread::spawn(move || {
                    lower_current_thread_priority();
                    let pool = rayon::ThreadPoolBuilder::new()
                        .start_handler(|_| lower_current_thread_priority())
                        .build()
                        .unwrap();
                    pool.install(|| renderer.render(tx_channel, cancel));
                }));
                self.start_timer = std::time::Instant::now();
            }

            match self.rx.try_recv() {
                Ok((frame_buffer, w, h)) => {
                    self.last_frame_buffer = frame_buffer;
                    self.pixels_size = [w, h];
                    self.iterations += 1;
                    self.elapsed = self.start_timer.elapsed().as_secs_f64();
                    self.pixels = match self.render_mode {
                        RenderMode::Pathtracing => {
                            tone_mapping(&self.last_frame_buffer, self.exposure, self.gamma)
                        }
                        _ => {
                            simple_tone_mapping(&self.last_frame_buffer, self.exposure, self.gamma)
                        }
                    };
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    eprintln!("Error receiving frame buffer: disconnected");
                }
            };

            if params_changed && !self.last_frame_buffer.is_empty() {
                self.pixels = match self.render_mode {
                    RenderMode::Pathtracing => {
                        tone_mapping(&self.last_frame_buffer, self.exposure, self.gamma)
                    }
                    _ => simple_tone_mapping(&self.last_frame_buffer, self.exposure, self.gamma),
                };
            }

            ui.add(
                egui::Image::new(egui::load::SizedTexture::new(
                    &ctx.load_texture(
                        "raytraced",
                        egui::ColorImage::from_rgba_unmultiplied(
                            self.pixels_size,
                            self.pixels.as_mut_slice(),
                        ),
                        Default::default(),
                    ),
                    size,
                ))
                .fit_to_exact_size(size),
            );

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
        4 => make_box_scene(),
        5 => make_three_spheres_scene(),
        6 => make_glass_and_mirror_scene(),
        7 => make_glass_sphere_scene(),
        _ => make_default_scene(),
    }
}

fn tone_mapping(frame_buffer: &Vec<Vec3>, exposure: f32, gamma: f32) -> Vec<u8> {
    let inv_gamma = 1.0 / gamma;
    let exposure_scale = 2.0f32.powf(exposure);
    let exposure_inv = compute_exposure_inv(frame_buffer) / exposure_scale;

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

fn simple_tone_mapping(frame_buffer: &Vec<Vec3>, _exposure: f32, _gamma: f32) -> Vec<u8> {
    let max = frame_buffer
        .iter()
        .flat_map(|v| v.to_array())
        .reduce(f32::max)
        .unwrap_or(0.01);

    frame_buffer
        .iter()
        .map(|x| (x / max).clamp(Vec3::ZERO, Vec3::ONE))
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
