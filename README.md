# rs-tracer

A simple ray tracing program built as a tinkering project to enhance Rust skills.

## Overview

This is a Cargo workspace containing two crates:

- **tracer-core** — Core ray tracing library with geometry primitives (spheres, planes, rays), materials, lighting, scene management, and rendering logic
- **tracer-render** — Application using SDL2 to render the scene to a window

## Features

- Ray-sphere and ray-plane intersection
- Phong lighting model (ambient, diffuse, specular)
- Multi-threaded rendering via `rayon`
- Real-time SDL2 window output
- Heap profiling with `dhat` (opt-in feature)
- Benchmarking with `hyperfine` (opt-in feature)

## Dependencies

- **glam** — Linear algebra (vectors, matrices)
- **rayon** — Data parallelism
- **sdl2** — Windowing and rendering
- **dhat** — Heap profiling (dev)
- **range2d** — 2D range utilities

## Building

```bash
# Debug build (with LTO for faster iteration)
cargo build

# Release build (optimized)
cargo build --release

# With heap profiling
cargo build --features dhat-heap

# With benchmarking
cargo build --features hyperfine
```

## Running

```bash
# Default (interactive SDL2 window)
cargo run --package tracer-render

# Benchmark mode (renders 100 frames and exits)
cargo run --package tracer-render --features hyperfine
```

## Project Structure

```
rs-tracer/
├── Cargo.toml              # Workspace root
├── tracer-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── entity/
│       │   ├── mod.rs
│       │   ├── actor.rs
│       │   ├── geometry/
│       │   │   ├── mod.rs
│       │   │   ├── ray.rs
│       │   │   ├── sphere.rs
│       │   │   └── plane.rs
│       │   ├── rendering/
│       │   │   ├── mod.rs
│       │   │   ├── material.rs
│       │   │   └── light.rs
│       │   └── scene.rs
│       └── rendering/
│           ├── mod.rs
│           ├── renderer.rs
│           ├── ray_emitter.rs
│           └── image_filter.rs
└── tracer-render/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── renderer.rs
```

## Scene Description

The demo scene renders:

- A green sphere at origin (radius 50)
- A blue sphere below (radius 20, centered at (0, -100, 0))
- A white directional light from above-right
- Camera at (0, 0, -10) looking toward +Z
- Light gray background (10, 10, 10)

## For Contributors

See [AGENTS.md](AGENTS.md) for implementation conventions, extension patterns, and common pitfalls.

## License

MIT