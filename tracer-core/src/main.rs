use glam::{Vec3, Vec4};

use crate::entity::geometry::plane::Plane;
use crate::entity::geometry::sphere::Sphere;
use crate::entity::rendering::light::Light;
use crate::entity::scene::Scene;
use crate::rendering::ray_emitter::RayEmitter;
use crate::rendering::renderer::Renderer;

mod entity;
mod rendering;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let renderer = Renderer::new(&video_subsystem, &sdl_context, 800, 800);
    let ray_emitter = RayEmitter::new(Vec3::new(0., 0., -49.), Vec3::new(0., 0., 1.), 800, 800);

    let mut scene: Scene = Scene::new();

    let light = Light::new(
        &Vec3::new(0., 400., 0.),
        &Vec3::new(0., -1., 0.),
        50.,
        Vec4::new(0., 255., 255., 1.),
    );
    scene.renderables.push(&light);

    let sphere: Sphere = Sphere::new(&Vec3::new(0., 100., 0.), 50., Vec4::new(0., 255., 0., 1.));
    scene.renderables.push(&sphere);

    let plane: Plane = Plane::new(
        &Vec3::new(0., 0., 10.),
        &Vec3::new(0., 0., -1.),
        &Vec4::new(0., 0., 255., 1.),
    );
    scene.renderables.push(&plane);

    loop {
        renderer.render(&ray_emitter, &mut scene, &light);
    }
}
