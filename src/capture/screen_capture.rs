use scrap::{ Capturer, Display };
use std::io::ErrorKind::WouldBlock;

use crate::texture::Texture;

pub struct ScreenCapture {
    capturer: Capturer,
    screen_width: usize,
    screen_height: usize,
    texture: Option<Texture>
}

impl ScreenCapture {
    pub fn new() -> Self {
        let display = Display::primary().expect("Failed to get primary display");

        let screen_width = display.width();
        let screen_height = display.height();

        let capturer = Capturer::new(display).expect("Failed to create capturer");
        let texture = Some(Texture::empty(screen_width as i32, screen_height as i32).expect("Failed to create texture"));

        Self { capturer, screen_width, screen_height, texture }
    }

    pub fn get_frame(&mut self, size: (usize, usize), pos: (usize, usize)) -> &Texture {
        let (width, height) = size;

        // Recreate texture if size changed
        let recreate = match &self.texture {
            Some(tex) => {
                let (tw, th) = tex.get_texture_size();

                tw != width as i32 || th != height as i32
            },
            None => true
        };

        if recreate {
            self.texture = Some(Texture::empty(width as i32, height as i32).expect("Failed to recreate texture"));
        }

        let texture = self.texture.as_ref().unwrap();

        // FIX: Crash happens if window crosses the screen border on X axis but not Y axis
        match self.capturer.frame() {
            Ok(frame) => {
                let stride = frame.len() / self.screen_height;

                // Update the texture with the captured frame
                texture.update_from_ptr(size, pos, stride, frame.as_ptr() as *const _);
            },

            Err(ref e) if e.kind() == WouldBlock => {},
            Err(e) => println!("Capture error: {e}")
        }

        texture
    }
}
