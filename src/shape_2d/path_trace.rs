use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;

pub mod cubic_bezier_curve;
pub mod line;
pub enum PathTrace2D {
    CubicBezierCurve2D(cubic_bezier_curve::CubicBezierCurve2D),
    Line2D(line::Line2D),
}

impl PathTrace2D {
    pub(crate) fn draw_raw(&self, base: &Object2D, graphics_2d: &mut Graphics2D) {
        match self { 
            PathTrace2D::CubicBezierCurve2D(s) => s.draw_raw(base, graphics_2d),
            PathTrace2D::Line2D(s) => s.draw_raw(base, graphics_2d),
        }
    }
}