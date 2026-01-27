use sdl2::{Sdl, VideoSubsystem, event::Event, keyboard::Keycode, video::Window};

use tracer_core::{
    entity::{
        geometry::RayType,
        rendering::light::Light,
        scene::{Renderable, Scene},
    },
    rendering::{image_filter, ray_emitter::RayEmitter},
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

    /// Draw each object on the window surface, from the furthest to the nearest.
    pub fn render(&self, ray_emitter: &RayEmitter, scene: &mut Scene, light: &Light) -> bool {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        {
            let mut surface = self.window.surface(&event_pump).unwrap();
            surface.enable_RLE();
            surface.with_lock_mut(|buffer: &mut [u8]| {
                ray_emitter
                    .rays
                    .iter()
                    .map(|ray| scene.render(ray, light, &RayType::Camera))
                    .enumerate()
                    .for_each(|it| match it.1 {
                        None => {
                            buffer[it.0*4..(it.0*4)+4].copy_from_slice(&[0, 0, 0, 1]);
                        }
                        Some(result) => {
                            buffer[it.0*4..(it.0*4)+4].copy_from_slice(&[
                                result.x as u8, 
                                result.y as u8, 
                                result.z as u8,
                                result.w as u8
                                ]);
                        }
                    });
                image_filter::apply_msaa_3x3_serial(self.w, buffer);
            });

            let _ = surface.finish();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }
        false
    }
}
