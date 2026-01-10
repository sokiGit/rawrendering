use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::path_trace::PathTrace2D;
use crate::shape_2d::Drawable;

pub struct VectorPath2D {
    pub base: Object2D,
    pub path: Vec<PathTrace2D>,
}

impl Drawable for VectorPath2D {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        for path_trace in self.path.iter() {
            path_trace.draw_raw(&self.base, graphics_2d);
        }
    }
}