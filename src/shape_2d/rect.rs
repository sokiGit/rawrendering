use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::Shape2D;
use crate::utils::color::Color;

#[derive(Default)]
pub(crate) struct Rect {
    pub(crate) base: Object2D,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) fill_color: Color,
    pub(crate) outline_color: Color,
    //TODO: pub(crate) outline_thickness: usize,
}

impl Shape2D for Rect {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        // No zero-width or zero-height rects
        if self.width == 0 || self.height == 0 { return; }

        let outline_alpha = self.outline_color.value >> 24 & 0xFF;

        // Check whether color is fully transparent, if so, don't draw fill
        if self.fill_color.value >> 24 & 0xFF != 0xFF {  // 0xFF000000 -> 0x000000FF (6 hex sh = 24 bin sh)
            // ... more efficiency? :3c mayhaps
            let (start_x, start_y, end_x, end_y ) = if outline_alpha == 0xFF {
                (1, 1, self.width - 1, self.height - 1)
            } else {
                (0, 0, self.width, self.height)
            };

            for x_px in start_x..end_x {
                for y_px in start_y..end_y {
                    graphics_2d._set_pixel(x_px, y_px, &self.fill_color, &self.base);
                }
            }
        }

        // Draw outline (if present)
        if outline_alpha != 0xFF {
            let width = self.width - 1;
            let height = self.height - 1;

            // top and bottom
            for x in 0..self.width {
                println!("Vertical rect border fill");
                graphics_2d._set_pixel(x, 0, &self.outline_color, &self.base);
                graphics_2d._set_pixel(x, height, &self.outline_color, &self.base);
            }
            
            // left and right
            for y in 1..self.height - 1 {
                println!("Horizontal rect border fill");
                graphics_2d._set_pixel(0, y, &self.outline_color, &self.base);
                graphics_2d._set_pixel(width, y, &self.outline_color, &self.base);
            }
        }
    }
}
