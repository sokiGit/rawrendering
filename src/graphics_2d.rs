use crate::canvas::Canvas;
use crate::object_2d::Object2D;
use crate::shape_2d::{Drawable, Shape2D};
use crate::utils::color::Color;

pub struct Graphics2D<'a> {
    pub canvas: &'a mut Canvas<'a>,
}

impl Graphics2D<'_> {
    pub fn draw_shape(&mut self, shape_2d: &Shape2D) {
        shape_2d.draw(self);
    }

    pub(crate) fn _set_pixel(&mut self, x: u32, y: u32, color: &Color, object_2d: &Object2D)
    {
            //TODO: Fix type conversion hell
            //TODO: Relative anchor (needs a way to get object bounds, eg. Drawable.bounds_size: Vec2<u32>)
            let relative_offset_as_absolute_x = (object_2d.relative_offset.x * self.canvas.width as f32) as i32;
            let relative_offset_as_absolute_y = (object_2d.relative_offset.y * self.canvas.height as f32) as i32;

            let new_x_offset = relative_offset_as_absolute_x + object_2d.offset.x - object_2d.anchor.x;
            let new_y_offset = relative_offset_as_absolute_y + object_2d.offset.y - object_2d.anchor.y;

            let x_pos = new_x_offset + x as i32;
            let y_pos = new_y_offset + y as i32;

            if x_pos < 0 || y_pos < 0  { return; }
            self.canvas.set_pixel(
                x_pos as u32,
                y_pos as u32,
                color
            );
    }
}