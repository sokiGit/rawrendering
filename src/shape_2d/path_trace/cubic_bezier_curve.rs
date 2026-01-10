use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::utils::color::Color;
use crate::utils::vec2::Vec2;

pub struct CubicBezierCurve2D {
    pub from: Vec2<i32>,
    pub to: Vec2<i32>,
    pub from_control_point: Vec2<i32>,
    pub to_control_point: Vec2<i32>,
    pub color: Color,
}

impl CubicBezierCurve2D{
    pub(crate) fn draw_raw(&self, base: &Object2D, graphics_2d: &mut Graphics2D) {
        //TODO: This is also legacy code in need of a re-make

        // x1, y1 = Start of curve
        // x1_c, y2_c = First 'Control Point', for (x1, y1), sets curvature at start of curve
        // x2, y2 = End of curve
        // x2_c, y2_c = Last 'Control Point', for (x2, y2), sets curvature at end of curve

        let x1 = self.from.x as f32;
        let y1 = self.from.y as f32;
        let x1_c = self.from_control_point.x as f32;
        let y1_c = self.from_control_point.y as f32;
        let x2 = self.to.x as f32;
        let y2 = self.to.y as f32;
        let x2_c = self.to_control_point.x as f32;
        let y2_c = self.to_control_point.y as f32;

        // This makes the lines more full and ensures that even the most extreme curves don't have gaps.
        // Multiplying it by 2x can lead to improvements, 4x as well but the returns diminish quickly
        let sample_points = (((x1 - x1_c) * (x1 - x1_c) + (y1 - y1_c) * (y1 - y1_c)).sqrt()
            + ((x1_c - x2_c) * (x1_c - x2_c) + (y1_c - y2_c) * (y1_c - y2_c)).sqrt()
            + ((x2_c - x2) * (x2_c - x2) + (y2_c - y2) * (y2_c - y2)).sqrt())
            * 4f32;

        // Constants, do not re-calculate
        let delta_inter_1_x = (x1_c - x1) / sample_points;
        let delta_inter_1_y = (y1_c - y1) / sample_points;

        let delta_inter_2_x = (x2 - x2_c) / sample_points;
        let delta_inter_2_y = (y2 - y2_c) / sample_points;

        let mut i = 0f32;
        loop {
            // Figure out position of pixel

            // Interpolate between: (x1, y1) -> (x1_c, y1_c), call that inter_1
            // Interpolate between: (x1_c, y1_c) -> (x2_c, y2_c); call that bridge
            // Interpolate between: (x2_c, y2_c) -> (x2, y2), call that inter_2
            // Interpolate between: inter_1 -> bridge, call that b_1
            // Interpolate between: bridge -> inter_2, call that b_2
            // Interpolate between: b_1 -> b_2, that is (x_px, y_px)

            let inter_1_x = x1 + delta_inter_1_x * i;
            let inter_1_y = y1 + delta_inter_1_y * i;

            let bridge_x = x1_c + (x2_c - x1_c) * i / sample_points;
            let bridge_y = y1_c + (y2_c - y1_c) * i / sample_points;

            let inter_2_x = x2_c + delta_inter_2_x * i;
            let inter_2_y = y2_c + delta_inter_2_y * i;

            let b_1_x = inter_1_x + (bridge_x - inter_1_x) * i / sample_points;
            let b_1_y = inter_1_y + (bridge_y - inter_1_y) * i / sample_points;

            let b_2_x = bridge_x + (inter_2_x - bridge_x) * i / sample_points;
            let b_2_y = bridge_y + (inter_2_y - bridge_y) * i / sample_points;

            let x_px = (b_1_x + (b_2_x - b_1_x) * i / sample_points) as u32;
            let y_px = (b_1_y + (b_2_y - b_1_y) * i / sample_points) as u32;

            graphics_2d._set_pixel(x_px, y_px, &self.color, base);

            if i >= sample_points {
                break;
            }
            i += 1f32;
        }
    }
}