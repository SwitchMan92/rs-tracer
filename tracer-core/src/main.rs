use glam::{Vec3A, Vec4};

use crate::entity::geometry::GeometryImpl;
use crate::entity::geometry::plane::Plane;
use crate::entity::geometry::sphere::Sphere;
use crate::entity::rendering::light::Light;
use crate::entity::scene::Scene;
use crate::rendering::ray_emitter::RayEmitter;
use crate::rendering::renderer::Renderer;

mod entity;
mod rendering;

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

    let mut scene: Scene = Scene::new(&Vec4::new(10., 10., 10., 1.));

    let light = Light::new(
        &Vec3A::new(200., 200., 50.),
        &Vec3A::new(0., -1., 0.),
        50.,
        Vec4::new(0., 255., 255., 1.),
    );

    let sphere: Sphere = Sphere::new(&Vec3A::new(0., 0., 0.), 50., Vec4::new(0., 255., 0., 1.));
    scene.renderables.push(GeometryImpl::Sphere(sphere));

    let plane: Plane = Plane::new(
        &Vec3A::new(0., -100., 0.),
        &Vec3A::new(0., 1., 1.),
        &Vec4::new(0., 0., 255., 1.),
    );
    scene.renderables.push(GeometryImpl::Plane(plane));

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
