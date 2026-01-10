use crate::utils::color::Color;
// TODO: Fix mess with using isize/usize/f32 as params
// Maybe the functions can be sorted into two categories:
// Low-level, using only usize for efficiency
// High-level, using isize and f32 to make things more comfortable to use

pub struct Canvas<'a> {
    pub pixels: &'a mut [u32],
    pub width: u32,
    pub height: u32,
}

impl<'a> Canvas<'a> {
    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Color) {
        if x < self.width && y < self.height {
            let alpha = (color.value >> 24 & 0xFF) as f32 / 255f32; // 0xFF00000000 -> 0x000000FF (6 hex sh = 24 bin sh), mask isn't strictly necessary for u32

            if alpha == 1f32 { return; } // Transparent, don't draw
            else if alpha == 0f32 {
                // Opaque, don't calculate color mixing since it's just the new color anyway
                self.pixels[(y * self.width + x) as usize] = color.value;
            } else {
                // Color is semi-transparent, color mixing needs to be calculated
                self.pixels[(y * self.width + x) as usize] = Color::from(self.get_pixel(x, y)).mix_overlapping_destroy_transparency(color).value; // ... this .. seems to kinda suck...
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize]
        } else {
            0x00000000
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

}
