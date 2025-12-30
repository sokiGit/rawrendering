use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::path_trace::PathTrace;
use crate::utils::color::Color;
use crate::utils::vec2::Vec2;

pub(crate) struct Line2D {
    pub(crate) from: Vec2<isize>,
    pub(crate) to: Vec2<isize>,
    pub(crate) color: Color,
}

impl PathTrace for Line2D {
    fn draw(&self, base: &Object2D, graphics_2d: &mut Graphics2D) {
        //TODO: Needs heavy optimisation, this is legacy code with minimal modification
        let x1 = self.from.x as f32;
        let y1 = self.from.y as f32;

        let x2 = self.to.x as f32;
        let y2 = self.to.y as f32;

        let dist_x = x1 - x2;
        let dist_y = y1 - y2;
        let sample_points = (dist_x * dist_x + dist_y * dist_y).sqrt();

        let delta_x = (x2 - x1) / sample_points;
        let delta_y = (y2 - y1) / sample_points;

        let mut i = 0f32;
        loop {  
            graphics_2d._set_pixel(
                (x1 + delta_x * i) as usize,
                (y1 + delta_y * i) as usize,
                &self.color,
                base
            );
            if i >= sample_points {
                break;
            }
            i += 1f32;
        }
    }
}