use crate::graphics_2d::Graphics2D;

// Export Shapes
pub(crate) mod rect;
pub(crate) mod circle;
pub(crate) mod vector_path;
pub(crate) mod path_trace;

// Define Trait
pub(crate) trait Shape2D {
    fn draw(&self, graphics_2d: &mut Graphics2D);
}