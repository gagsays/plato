extern crate png;

use std::fs::File;
use png::HasParameters;
use failure::{Error, ResultExt};
use super::{Framebuffer, UpdateMode};
use color::WHITE;
use geom::{Rectangle, lerp};

#[derive(Debug, Clone)]
pub struct Pixmap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Pixmap {
    pub fn new(width: u32, height: u32) -> Pixmap {
        let len = (width * height) as usize;
        Pixmap {
            width,
            height,
            data: vec![WHITE; len],
        }
    }
}

impl Framebuffer for Pixmap {
    fn set_pixel(&mut self, x: u32, y: u32, color: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let addr = (y * self.width + x) as usize;
        self.data[addr] = color;
    }

    fn set_blended_pixel(&mut self, x: u32, y: u32, color: u8, alpha: f32) {
        if alpha >= 1.0 {
            self.set_pixel(x, y, color);
            return;
        }
        if x >= self.width || y >= self.height {
            return;
        }
        let addr = (y * self.width + x) as usize;
        let blended_color = lerp(self.data[addr] as f32, color as f32, alpha) as u8;
        self.data[addr] = blended_color;
    }

    fn invert_region(&mut self, rect: &Rectangle) {
        for y in rect.min.y..rect.max.y {
            for x in rect.min.x..rect.max.x {
                let addr = (y * self.width as i32 + x) as usize;
                let color = 255 - self.data[addr];
                self.data[addr] = color;
            }
        }
    }

    fn update(&mut self, _rect: &Rectangle, _mode: UpdateMode) -> Result<u32, Error> {
        Ok(1)
    }

    fn wait(&self, _: u32) -> Result<i32, Error> {
        Ok(1)
    }

    fn save(&self, path: &str) -> Result<(), Error> {
        let (width, height) = self.dims();
        let file = File::create(path).context("Can't create output file.")?;
        let mut encoder = png::Encoder::new(file, width, height);
        encoder.set(png::ColorType::Grayscale).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().context("Can't write header.")?;
        writer.write_image_data(&self.data).context("Can't write data to file.")?;
        Ok(())
    }

    fn toggle_inverted(&mut self) {
    }

    fn toggle_monochrome(&mut self) {
    }

    fn dims(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
