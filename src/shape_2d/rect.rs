use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::Shape2D;

#[derive(Default)]
pub(crate) struct Rect {
    pub(crate) base: Object2D,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) fill_color: u32,
    pub(crate) outline_color: u32,
    //TODO: pub(crate) fill_thickness: usize,
}

impl Shape2D for Rect {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        // Check whether there's something to draw on
        // No zero-width or zero-height rects
        if graphics_2d.canvas.is_none() || self.width == 0 || self.height == 0 { return; }

        let is_outline_transparent = self.outline_color >> 24 != 0xFF;

        // Check whether color is fully transparent, if so, don't draw fill
        if self.fill_color >> 24 != 0xFF {  // 0xFF000000 -> 0x000000FF (6 hex sh = 24 bin sh)
            // ... more efficiency? :3c mayhaps
            let (start_x, start_y, end_x, end_y ) = if is_outline_transparent {
                (1, 1, self.width - 1, self.height - 1)
            } else {
                (0, 0, self.width, self.height)
            };

            for x_px in start_x..end_x {
                for y_px in start_y..end_y {
                    graphics_2d._set_pixel(x_px, y_px, self.fill_color, &self.base);
                }
            }
        }

        // Repeat for outline (if present)
        if is_outline_transparent {
            // Uhm ... efficiency? :3c
            if self.height > 1 {
                for i in 0..self.width {
                    graphics_2d._set_pixel(i, 0, self.outline_color, &self.base);
                    graphics_2d._set_pixel(i, self.height - 1, self.outline_color, &self.base);
                }
            } else {
                for i in 0..self.width {
                    graphics_2d._set_pixel(i, 0, self.outline_color, &self.base);
                }
            }

            if self.height > 1 {
                for i in 0..self.width {
                    graphics_2d._set_pixel(0, i, self.outline_color, &self.base);
                    graphics_2d._set_pixel(self.width - 1, i, self.outline_color, &self.base);
                }
            }
            else {
                for i in 0..self.width {
                    graphics_2d._set_pixel(0, i, self.outline_color, &self.base);
                }
            }
        }
    }

    fn get_object_data(&self) -> &Object2D { &self.base }
}
