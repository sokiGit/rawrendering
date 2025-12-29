use crate::canvas::Canvas;
use crate::object_2d::Object2D;
use crate::shape_2d::Shape2D;

pub(crate) struct Graphics2D<'a> {
    pub(crate) canvas: &'a mut Option<Canvas<'a>>,
}

impl Graphics2D<'_> {
    pub(crate) fn draw_shape<T: Shape2D>(&mut self, shape_2d: &T) {
        if let Some(canvas) = self.canvas.as_mut() {
            shape_2d.draw(self);
        }
    }

    pub(crate) fn _set_pixel(&mut self, x: usize, y: usize, color: u32, object_2d: &Object2D)
    {
        if let Some(canvas) = self.canvas {
            //TODO: Fix type conversion hell
            //TODO: Relative anchor (needs a way to get object bounds, eg. Shape2D.bounds_size: Vec2<usize>)
            let relative_offset_as_absolute_x = (object_2d.relative_offset.x * canvas.width as f32) as isize;
            let relative_offset_as_absolute_y = (object_2d.relative_offset.y * canvas.height as f32) as isize;

            let new_x_offset = relative_offset_as_absolute_x + object_2d.offset.x - object_2d.anchor.x;
            let new_y_offset = relative_offset_as_absolute_y + object_2d.offset.x - object_2d.anchor.y;

            let x_pos = new_x_offset + x as isize;
            let y_pos = new_y_offset + y as isize;

            if x_pos < 0 || y_pos < 0  { return; }
            canvas.set_pixel(
                x_pos as usize,
                y_pos as usize,
                color
            )
        }
    }
}