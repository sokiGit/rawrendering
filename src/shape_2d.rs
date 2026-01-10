use crate::graphics_2d::Graphics2D;

// Export Shapes
pub mod rect;
pub mod circle;
pub mod vector_path;
pub mod path_trace;

// Define Trait
pub(crate) trait Drawable {
    fn draw(&self, graphics_2d: &mut Graphics2D);
}

pub enum Shape2D {
    Rect(rect::Rect),
    Circle(circle::Circle),
    VectorPath2D(vector_path::VectorPath2D),
}

impl Drawable for Shape2D {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        match self { 
            Shape2D::Rect(s) => s.draw(graphics_2d),
            Shape2D::Circle(s) => s.draw(graphics_2d),
            Shape2D::VectorPath2D(s) => s.draw(graphics_2d),
        }
    }
}