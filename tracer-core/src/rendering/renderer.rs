use sdl2::{Sdl, VideoSubsystem, event::Event, keyboard::Keycode, video::Window};
use std::process;

use crate::{
    entity::{
        geometry::RayType,
        rendering::light::Light,
        scene::{Renderable, Scene},
    },
    rendering::ray_emitter::RayEmitter,
};

/// Structure in charge of managing the window and the window's render target.
pub struct Renderer<'a> {
    window: Window,
    sdl_context: &'a Sdl,
    w: usize,
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

        let uswidth = width as usize;

        Self {
            window,
            sdl_context,
            w: uswidth,
        }
    }

    /// Reorder the scene's objects using the depth of the camera (z-order),
    /// then draw each object on the window surface, from the furthest to the nearest.
    pub fn render(&self, ray_emitter: &RayEmitter, scene: &mut Scene, light: &Light) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        {
            let mut surface = self.window.surface(&event_pump).unwrap();
            surface.enable_RLE();
            surface.with_lock_mut(|buffer: &mut [u8]| {
                let mut temp_buffer = vec![0; buffer.len()];
                temp_buffer.clone_from_slice(buffer);

                ray_emitter
                    .rays
                    .iter()
                    .map(|ray| scene.render(ray, light, &RayType::Camera))
                    .enumerate()
                    .for_each(|it| match it.1 {
                        None => {
                            temp_buffer[it.0 * 4] = 0;
                            temp_buffer[it.0 * 4 + 1] = 0;
                            temp_buffer[it.0 * 4 + 2] = 0;
                            temp_buffer[it.0 * 4 + 3] = 1;
                        }
                        Some(result) => {
                            temp_buffer[it.0 * 4] = result.x as u8;
                            temp_buffer[it.0 * 4 + 1] = result.y as u8;
                            temp_buffer[it.0 * 4 + 2] = result.z as u8;
                            temp_buffer[it.0 * 4 + 3] = result.w as u8;
                        }
                    });

                let x_offset = self.w * 4;
                let slice_end = buffer.len() - x_offset - 4;

                (x_offset + 4..slice_end).for_each(|x| {
                    buffer[x] = ((temp_buffer[x - x_offset - 4] as u16
                        + temp_buffer[x - x_offset] as u16
                        + temp_buffer[x - x_offset + 4] as u16
                        + temp_buffer[x - 4] as u16
                        + temp_buffer[x] as u16
                        + temp_buffer[x + 4] as u16
                        + temp_buffer[x + x_offset - 4] as u16
                        + temp_buffer[x + x_offset] as u16
                        + temp_buffer[x + x_offset + 4] as u16)
                        / 9) as u8;
                });
                // buffer.copy_from_slice(temp_buffer.as_slice());
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
