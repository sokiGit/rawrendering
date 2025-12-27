use crate::draw::polygon::Polygon;

pub mod polygon;

pub struct Canvas<'a> {
    pub pixels: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}

impl<'a> Canvas<'a> {
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
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

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for x_px in x..x + w {
            for y_px in y..y+ h {
                self.put_pixel(x_px, y_px, color);
            }
        }
    }

    pub fn draw_rect_outline(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        if w > 0 {
            for x_px in x..x +w {
                //NOTE: Puts pixels even if h == 0, this results in h = 1 and h = 0 looking the same
                self.put_pixel(x_px, y, color);
                if h > 0 {
                    self.put_pixel(x_px, y + h - 1, color);
                }
            }
        }

        if h > 0 {
            for y_px in y..y +h {
                //NOTE: Puts pixels even if w == 0, this results in w = 1 and w = 0 looking the same
                self.put_pixel(x, y_px, color);
                if w > 0 {
                    self.put_pixel(x + w - 1, y_px, color);
                }
            }
        }
    }

    pub fn draw_rect_with_transparency(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for x_px in x..x + w {
            for y_px in y..y + h {
                let old_color = self.get_pixel(x_px, y_px);
                let old_r = (old_color >> 16) & 0xFF;   // 0xFF0000 -> 0x0000FF (4 hex sh = 16 bin sh), requires mask
                let old_g = (old_color >> 8) & 0xFF;    // 0x00FF00 -> 0x0000FF (2 hex sh = 8 bin sh), requires mask
                let old_b = old_color & 0xFF;           // 0x0000FF -> 0x0000FF (0 hex sh = 0 bin sh), requires mask

                let new_r = color >> 16 & 0xFF;
                let new_g = (color >> 8) & 0xFF;
                let new_b = color & 0xFF;

                let transparency = (color >> 24) as f32 / 255f32; // 0xFF00000000 -> 0x000000FF (6 hex sh = 24 bin sh), no mask required, u32 ends here

                // Additive color mixing (not desirable, but good PoC for now)
                let fin_r = (old_r + (new_r as f32 * transparency) as u32) & 0xFF;
                let fin_g = (old_g + (new_g as f32 * transparency) as u32) & 0xFF;
                let fin_b = (old_b + (new_b as f32 * transparency) as u32) & 0xFF;

                let fin_color = fin_r << 16 | fin_g << 8 | fin_b;

                self.put_pixel(x_px, y_px, fin_color);
            }
        }
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        // I hate the type conversions, but I'm happy I figured it out on my own, TODO: Fix
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let x2 = x2 as isize;
        let y2 = y2 as isize;

        let dist_x = x1 - x2;
        let dist_y = y1 - y2;
        let sample_points = (dist_x * dist_x + dist_y * dist_y).isqrt() as f32;

        let delta_x = (x2 - x1) as f32 / sample_points;
        let delta_y = (y2 - y1) as f32 / sample_points;

        let mut i = 0f32;
        loop {
            self.put_pixel(
                (x1 + (delta_x * i) as isize) as usize,
                (y1 + (delta_y * i) as isize) as usize,
                color
            );
            if i == sample_points { break; }
            i += 1f32;
        }
    }

    pub fn draw_polygon_outline(&mut self, polygon: Polygon<'a>) {
        let path_len = polygon.path.len();
        if path_len < 2 { return; }

        for i in 0..path_len {
            if i < path_len - 1 {
                let (x1, y1) = polygon.path[i];
                let (x2, y2) = polygon.path[i + 1];
                self.draw_line(x1, y1, x2, y2, polygon.color);
            } else if polygon.closed {
                let (x1, y1) = polygon.path[i];
                let (x2, y2) = polygon.path[0];
                self.draw_line(x1, y1, x2, y2, polygon.color);
            }
        }
    }
}