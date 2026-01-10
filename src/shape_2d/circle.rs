use std::collections::HashSet;
use std::f32::consts::PI;
use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::Drawable;
use crate::utils::color::Color;
use crate::utils::vec2::Vec2;

#[derive(Default)]
pub struct Circle {
    pub base: Object2D,
    pub radius: u32,
    pub fill_color: Color,
    pub outline_color: Color,
    pub outline_thickness: u32
}

impl Drawable for Circle {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        for x in 0..self.radius*2 {
            for y in 0..self.radius*2 {
                let x_cmp = x as i32 - self.radius as i32;
                let y_cmp = y as i32 - self.radius as i32;

                let square = (x_cmp*x_cmp)+(y_cmp*y_cmp);

                if square <= (self.radius * self.radius) as i32 {
                    if square >= ((self.radius - self.outline_thickness) * (self.radius - self.outline_thickness)) as i32 {
                        graphics_2d._set_pixel(x, y, &self.fill_color.mix_overlapping_destroy_transparency(&self.outline_color), &self.base);
                    } else {
                        graphics_2d._set_pixel(x , y, &self.fill_color, &self.base);
                    }
                }
            }
        }
    }
}