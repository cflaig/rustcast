# rustcast

`rustcast` is an interactive renderer developed in Rust. This project serves as a learning platform for various rendering techniques, ranging from simple normal visualizations to physically-based Monte Carlo path tracing.

## Features

- **Multiple Rendering Modes:**
  - **Normals**: Visualization of surface normals for geometry verification.
  - **Raycast**: Simple shading with ambient and diffuse components.
  - **Raytrace**: Classic recursive ray tracing with reflections and refractions (based on Snell's law).
  - **Pathtracing**: Monte Carlo path tracing for realistic global illumination and soft shadows.
- **Interactive User Interface:** An integrated GUI (based on `eframe`/`egui`) allows for real-time switching between scenes and rendering modes.
- **Predefined Scenes:** Includes classic setups like the **Cornell Box**, cylinder scenes, and various test scenarios for light and shadow.
- **High Performance:** Calculations are parallelized using `rayon` to optimally utilize CPU cores.
- **Precise Mathematics:** Utilizes `glam` for high-performance vector calculations.

## Installation & Execution

Ensure you have the latest version of [Rust](https://www.rust-lang.org/) installed.

```bash
# Clone the repository
git clone <repository-url>
cd rustcast

# Start the app in release mode (recommended for performance)
cargo run --release
```

## Project Structure

- `src/main.rs`: Application entry point and `eframe` GUI definition.
- `src/renderer.rs`: Implementation of core algorithms (Raycasting, Raytracing, Pathtracing).
- `src/camera.rs`: Logic for the virtual camera and ray generation.
- `src/scenes.rs`: Definition of various test scenes.
- `src/shape/`: Contains definitions for geometric primitives like spheres, planes, and boxes.
- `docs/`: Mathematical derivations and additional documentation (e.g., on ray refraction).
- `visualize_sphere.py`: A Python script for visualizing the uniform distribution of random points on a sphere's surface.

## Mathematical Background

For those interested, the `docs/` folder contains detailed mathematical derivations:
- [Snell's refraction in vector form (English)](docs/refraction-derivation-en.md)
- [Snellius-Brechung in Vektorform (Deutsch)](docs/refraction-derivation.md)

## License

This project is licensed under the MIT License (or any other license of your choice).
