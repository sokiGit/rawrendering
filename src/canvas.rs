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

    pub fn draw_rect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        for x_px in x1..x2 {
            for y_px in y1..y2 {
                self.set_pixel(x_px, y_px, color);
            }
        }
    }

    pub fn draw_rect_outline(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        let from_x = x1.min(x2);
        let from_y = y1.min(y2);

        let to_x = x1.max(x2);
        let to_y = y1.max(y2);

        if x1 != x2 {
            // Place horizontal lines ==
            for x_px in from_x..to_x {
                // Upper line
                self.set_pixel(x_px, from_y, color);

                // Lower line
                if y1 != y2 {
                    self.set_pixel(x_px, to_y.max(1) - 1, color);
                }
            }
        }

        if y1 != y2 {
            // Place vertical lines ||
            for y_px in from_y..to_y {
                // Left line
                self.set_pixel(from_x, y_px, color);

                // Right line
                if x1 != x2 {
                    self.set_pixel(to_x.max(1) - 1, y_px, color);
                }
            }
        }
    }

    pub fn draw_rect_with_transparency(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        color: u32,
    ) {
        let from_x = x1.min(x2);
        let from_y = y1.min(y2);

        let to_x = x1.max(x2);
        let to_y = y1.max(y2);

        for x_px in from_x..to_x {
            for y_px in from_y..to_y {
                let old_color = self.get_pixel(x_px, y_px);
                let old_r = (old_color >> 16) & 0xFF; // 0xFF0000 -> 0x0000FF (4 hex sh = 16 bin sh), requires mask
                let old_g = (old_color >> 8) & 0xFF; // 0x00FF00 -> 0x0000FF (2 hex sh = 8 bin sh), requires mask
                let old_b = old_color & 0xFF; // 0x0000FF -> 0x0000FF (0 hex sh = 0 bin sh), requires mask

                let new_r = color >> 16 & 0xFF;
                let new_g = (color >> 8) & 0xFF;
                let new_b = color & 0xFF;

                let transparency = (color >> 24) as f32 / 255f32; // 0xFF00000000 -> 0x000000FF (6 hex sh = 24 bin sh), no mask required, u32 ends here

                // Additive color mixing (not desirable, but good PoC for now)
                let fin_r = (old_r + (new_r as f32 * transparency) as u32) & 0xFF;
                let fin_g = (old_g + (new_g as f32 * transparency) as u32) & 0xFF;
                let fin_b = (old_b + (new_b as f32 * transparency) as u32) & 0xFF;

                let fin_color = fin_r << 16 | fin_g << 8 | fin_b;

                self.set_pixel(x_px, y_px, fin_color);
            }
        }
    }

    pub fn draw_line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: u32) {
        let x1 = x1 as f32;
        let y1 = y1 as f32;

        let x2 = x2 as f32;
        let y2 = y2 as f32;

        let dist_x = x1 - x2;
        let dist_y = y1 - y2;
        let sample_points = (dist_x * dist_x + dist_y * dist_y).sqrt();

        let delta_x = (x2 - x1) / sample_points;
        let delta_y = (y2 - y1) / sample_points;

        let mut i = 0f32;
        loop {
            self.set_pixel(
                (x1 + delta_x * i) as usize,
                (y1 + delta_y * i) as usize,
                color,
            );
            if i >= sample_points {
                break;
            }
            i += 1f32;
        }
    }

    pub fn draw_polygon_outline(&mut self, polygon: Polygon<'a>) {
        let path_len = polygon.path.len();
        if path_len < 2 {
            return;
        }

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

    pub fn draw_cubic_bezier_curve_outline(
        &mut self,
        x1: isize,
        y1: isize,
        x1_c: isize,
        y1_c: isize,
        x2: isize,
        y2: isize,
        x2_c: isize,
        y2_c: isize,
        color: u32,
    ) {
        // x1, y1 = Start of curve
        // x1_c, y2_c = First 'Control Point', for (x1, y1), sets curvature at start of curve
        // x2, y2 = End of curve
        // x2_c, y2_c = Last 'Control Point', for (x2, y2), sets curvature at end of curve

        let x1 = x1 as f32;
        let y1 = y1 as f32;
        let x1_c = x1_c as f32;
        let y1_c = y1_c as f32;
        let x2 = x2 as f32;
        let y2 = y2 as f32;
        let x2_c = x2_c as f32;
        let y2_c = y2_c as f32;

        // This makes the lines more full and ensures that even the most extreme curves don't have gaps.
        // Multiplying it by 2x can lead to improvements, 4x as well but the returns diminish quickly
        let sample_points = (((x1 - x1_c) * (x1 - x1_c) + (y1 - y1_c) * (y1 - y1_c)).sqrt()
            + ((x1_c - x2_c) * (x1_c - x2_c) + (y1_c - y2_c) * (y1_c - y2_c)).sqrt()
            + ((x2_c - x2) * (x2_c - x2) + (y2_c - y2) * (y2_c - y2)).sqrt())
            * 4f32;

        // Constants, do not re-calculate
        let delta_inter_1_x = (x1_c - x1) / sample_points;
        let delta_inter_1_y = (y1_c - y1) / sample_points;

        let delta_inter_2_x = (x2 - x2_c) / sample_points;
        let delta_inter_2_y = (y2 - y2_c) / sample_points;

        let mut i = 0f32;
        loop {
            // Figure out position of pixel

            // Interpolate between: (x1, y1) -> (x1_c, y1_c), call that inter_1
            // Interpolate between: (x1_c, y1_c) -> (x2_c, y2_c); call that bridge
            // Interpolate between: (x2_c, y2_c) -> (x2, y2), call that inter_2
            // Interpolate between: inter_1 -> bridge, call that b_1
            // Interpolate between: bridge -> inter_2, call that b_2
            // Interpolate between: b_1 -> b_2, that is (x_px, y_px)

            let inter_1_x = x1 + delta_inter_1_x * i;
            let inter_1_y = y1 + delta_inter_1_y * i;

            let bridge_x = x1_c + (x2_c - x1_c) * i / sample_points;
            let bridge_y = y1_c + (y2_c - y1_c) * i / sample_points;

            let inter_2_x = x2_c + delta_inter_2_x * i;
            let inter_2_y = y2_c + delta_inter_2_y * i;

            let b_1_x = inter_1_x + (bridge_x - inter_1_x) * i / sample_points;
            let b_1_y = inter_1_y + (bridge_y - inter_1_y) * i / sample_points;

            let b_2_x = bridge_x + (inter_2_x - bridge_x) * i / sample_points;
            let b_2_y = bridge_y + (inter_2_y - bridge_y) * i / sample_points;

            let x_px = (b_1_x + (b_2_x - b_1_x) * i / sample_points) as usize;
            let y_px = (b_1_y + (b_2_y - b_1_y) * i / sample_points) as usize;

            self.set_pixel(x_px, y_px, color);

            if i >= sample_points {
                break;
            }
            i += 1f32;
        }
    }

    pub fn draw_circle(&mut self, mid_x: usize, mid_y: usize, r: usize, color: u32) {
        for x in -(r as isize)..r as isize {
            for y in -(r as isize)..r as isize {
                if x * x + y * y <= (r * r) as isize {
                    self.set_pixel(
                        (mid_x as isize + x) as usize,
                        (mid_y as isize + y) as usize,
                        color,
                    );
                }
            }
        }
    }

    pub fn draw_circle_outline(&mut self, mid_x: usize, mid_y: usize, r: usize, color: u32) {
        let r_f32 = r as f32;

        let sample_points = 2f32 * PI * r_f32 * r_f32.sqrt(); // Maybe try not to use sqrt, but for now it's to make scaling consistent, also it can grow pretty quickly
        let mut i = 0f32;

        loop {
            let px_x = (i.cos() * r_f32) + mid_x as f32;
            let px_y = (i.sin() * r_f32) + mid_y as f32;

            if px_x >= 0f32 && px_y >= 0f32 {
                self.set_pixel(px_x as usize, px_y as usize, color);
            }


            if i >= sample_points {
                break;
            }
            i += 1f32;
        }
    }
}
