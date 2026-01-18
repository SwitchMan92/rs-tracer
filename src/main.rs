use geometry::sphere::Sphere;
use glam::{Vec3, Vec4};
use sdl2::sys::Window;

use crate::geometry::actor::Actor;
use crate::geometry::scene::Scene;
use crate::rendering::light::Light;
use crate::rendering::ray_emitter::RayEmitter;
use crate::rendering::renderer::Renderer;

mod geometry;
mod rendering;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let renderer = Renderer::new(&video_subsystem, &sdl_context, 800, 800);

    let sphere: Sphere = Sphere {
        actor: Actor::new(Vec3::new(100., 0., 0.)),
        radius: 100.,
        color: Vec4::new(0., 255., 0., 1.),
    };

    let mut scene: Scene = Scene::new();
    scene.renderables.push(Box::new(sphere));

    let ray_emitter = RayEmitter::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), 800, 800);

    let light = Light {
        actor: Actor::new(Vec3::new(10., 200., 0.)),
        direction: Vec3::new(0., 0., 1.),
        radius: 50.,
        color: Vec4::new(0., 255., 255., 1.),
    };

    let light_box = Box::new(light.clone());
    scene.renderables.push(light_box.clone());

    loop {
        renderer.render(&ray_emitter, &scene, &light_box);
    }
}
