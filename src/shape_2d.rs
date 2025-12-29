use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;

pub(crate) mod rect;

pub(crate) trait Shape2D {
    fn draw(&self, graphics_2d: &mut Graphics2D);
    fn get_object_data(&self) -> &Object2D;
}