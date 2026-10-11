# rs-tracer — Agent Instructions

> See [README.md](README.md) for project overview, features, dependencies, build/run commands, and full directory tree.

## Key Types & Traits

| Type | Location | Purpose |
|------|----------|---------|
| `Geometry` trait | `entity/geometry/mod.rs` | `intersect`, `get_surface_normal`, `get_material` |
| `GeometryImpl` enum | `entity/geometry/mod.rs` | Dispatches to Plane/Sphere/Light |
| `RayEmitter` | `rendering/ray_emitter.rs` | Generates camera rays for resolution |
| `Scene` | `entity/scene.rs` | Holds renderables, `render(ray, light, ...)` |
| `Light` | `entity/rendering/light.rs` | Position, direction, intensity, color |
| `MaterialType` | `entity/rendering/material.rs` | Diffuse/Reflective/Refractive enum |
| `Renderer` | `tracer-render/src/renderer.rs` | SDL2 window, parallel ray tracing via rayon |

## Conventions

- **Parallelism**: Ray tracing uses `rayon::prelude::*` (`par_iter`) in `renderer.rs`
- **Math**: `glam` types (`Vec3A`, `Vec4`) throughout
- **Coordinates**: Right-handed, Z-forward camera at (0,0,-10) looking +Z
- **Color**: `Vec4` as RGBA 0–255 in demo; internal uses 0–1 floats
- **Tests**: `#[cfg(test)]` modules in same file (sphere.rs, plane.rs, ray_emitter.rs)
- **Features**: `dhat-heap` (global allocator), `hyperfine` (benchmark loop)

## Adding Geometry

1. Create `entity/geometry/<name>.rs` implementing `Geometry` trait
2. Add variant to `GeometryImpl` enum in `mod.rs`
3. Implement `ActorTrait` and `Geometry` match arms for new variant
4. Export from `mod.rs`

## Adding Materials

Extend `MaterialType` enum in `entity/rendering/material.rs` and handle in `Geometry::intersect` implementations.

## Common Pitfalls

- Don't forget `range2d` dependency for 2D ranges (used in ray_emitter)
- SDL2 requires `libsdl2-dev` / `SDL2-devel` system package
- Release profile uses `lto = true`, `codegen-units = 1`, `panic = "abort"`
- Dev profile also uses LTO for faster iteration (unusual but intentional)

## Directory-Specific AGENTS.md

None currently. Create `tracer-core/AGENTS.md` or `tracer-render/AGENTS.md` if crate-specific conventions emerge.