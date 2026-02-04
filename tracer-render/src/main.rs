use glam::{Vec3A, Vec4};

use renderer::Renderer;
use tracer_core::entity::geometry::GeometryType;
use tracer_core::entity::geometry::plane::Plane;
use tracer_core::entity::geometry::sphere::Sphere;
use tracer_core::entity::rendering::light::Light;
use tracer_core::entity::rendering::material::{
    ColorMaterial, DiffuseMaterial, MaterialMixer, MaterialType,
};
use tracer_core::entity::scene::Scene;
use tracer_core::rendering::ray_emitter::RayEmitter;

mod renderer;

const RESOLUTION: (u32, u32) = (1000, 1000);

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let renderer = Renderer::new(&video_subsystem, &sdl_context, RESOLUTION.0, RESOLUTION.1);
    let camera_emitter = RayEmitter::new(
        Vec3A::new(0., 0., -10.),
        Vec3A::new(0., 0., 1.),
        RESOLUTION.0,
        RESOLUTION.1,
    );

    let mut scene: Scene = Scene::new(&Vec4::new(0., 0., 0., 1.));

    let mut material_mixer = MaterialMixer::new();
    material_mixer
        .materials
        .push(MaterialType::Color(ColorMaterial::new(Vec4::new(
            0., 1., 0., 1.,
        ))));
    material_mixer
        .materials
        .push(MaterialType::Diffuse(DiffuseMaterial::new(0.5)));

    let sphere: Sphere = Sphere::new(
        &Vec3A::new(0., 0., 0.),
        50.,
        &MaterialType::Mixer(material_mixer),
    );
    scene.renderables.push(GeometryType::Sphere(sphere));

    let plane: Plane = Plane::new(
        &Vec3A::new(0., -100., 0.),
        &Vec3A::new(0., 1., 1.),
        &MaterialType::Color(ColorMaterial::new(Vec4::new(0., 0., 1., 1.))),
    );
    scene.renderables.push(GeometryType::Plane(plane));

    let light = Light::new(
        &Vec3A::new(200., 200., 50.),
        &Vec3A::new(0., -1., 0.),
        50.,
        &MaterialType::Color(ColorMaterial::new(Vec4::new(0., 1., 1., 1.))),
    );

    loop {
        #[cfg(feature = "hyperfine")]
        {
            for _ in 0..100 {
                renderer.render(&camera_emitter, &mut scene, &light);
            }
            break;
        }

        if renderer.render(&camera_emitter, &mut scene, &light) {
            break;
        }
    }
}
