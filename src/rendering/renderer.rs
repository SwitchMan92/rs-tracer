use sdl2::{Sdl, VideoSubsystem, event::Event, keyboard::Keycode, video::Window};
use std::{cmp::Reverse, process};

use crate::{
    geometry::{Geometry, actor::ActorWithGeometry, scene::Scene},
    rendering::{light::Light, ray_emitter::RayEmitter},
};

pub struct Renderer<'a> {
    window: Window,
    sdl_context: &'a Sdl,
}

impl<'a> Renderer<'a> {
    pub fn new(
        video_subsystem: &'a VideoSubsystem,
        sdl_context: &'a Sdl,
        width: u32,
        height: u32,
    ) -> Self {
        let window = video_subsystem
            .window("raytracer", width, height)
            .position_centered()
            .build()
            .unwrap();

        Self {
            window: window,
            sdl_context: sdl_context,
        }
    }

    pub fn render(&self, ray_emitter: &RayEmitter, scene: &mut Scene, light: &Light) {
        scene.renderables.sort_by(|a, b| {
            (a.get_position() - ray_emitter.position)
                .length()
                .partial_cmp(&(b.get_position() - ray_emitter.position).length())
                .unwrap()
        });

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        {
            let mut surface = self.window.surface(&event_pump).unwrap();
            surface.enable_RLE();
            surface.with_lock_mut(|buffer: &mut [u8]| {
                ray_emitter
                    .rays
                    .iter()
                    .map(|ray| scene.collide(ray, light))
                    .enumerate()
                    .for_each(|it| {
                        buffer[it.0 * 4] = it.1.x as u8;
                        buffer[it.0 * 4 + 1] = it.1.y as u8;
                        buffer[it.0 * 4 + 2] = it.1.z as u8;
                        buffer[it.0 * 4 + 3] = 1;
                    });
            });

            let _ = surface.finish();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => process::exit(1),
                _ => {}
            }
        }
    }
}
