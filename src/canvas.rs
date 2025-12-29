use crate::canvas::polygon::Polygon;
use std::f32::consts::PI;

// TODO: Fix mess with using isize/usize/f32 as params
// Maybe the functions can be sorted into two categories:
// Low-level, using only usize for efficiency
// High-level, using isize and f32 to make things more comfortable to use

pub mod polygon;

pub struct Canvas<'a> {
    pub pixels: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}

impl<'a> Canvas<'a> {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        // TODO: Implement color mixing in set_pixel
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            0x00000000
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

}
