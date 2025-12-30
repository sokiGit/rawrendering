use std::collections::HashSet;
use std::f32::consts::PI;
use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::Shape2D;
use crate::utils::color::Color;
use crate::utils::vec2::Vec2;

#[derive(Default)]
pub(crate) struct Circle {
    pub(crate) base: Object2D,
    pub(crate) radius: usize,
    pub(crate) fill_color: Color,
    pub(crate) outline_color: Color,
    //TODO: pub(crate) outline_thickness: usize, (Maybe a better name than thickness)
}

impl Circle {
    fn _draw_8_points(&self, cx: isize, cy: isize, x: isize, y: isize, hash_set: &mut HashSet<Vec2<usize>>) {
        hash_set.insert(Vec2::new((cx + x) as usize, (cy + y) as usize));
        hash_set.insert(Vec2::new((cx - x) as usize, (cy + y) as usize));
        hash_set.insert(Vec2::new((cx + x) as usize, (cy - y) as usize));
        hash_set.insert(Vec2::new((cx - x) as usize, (cy - y) as usize));

        hash_set.insert(Vec2::new((cx + y) as usize, (cy + x) as usize));
        hash_set.insert(Vec2::new((cx - y) as usize, (cy + x) as usize));
        hash_set.insert(Vec2::new((cx + y) as usize, (cy - x) as usize));
        hash_set.insert(Vec2::new((cx - y) as usize, (cy - x) as usize));
    }
}

impl Shape2D for Circle {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        // No zero-radius circles
        if self.radius == 0 { return; }

        let mut outline_points: HashSet<Vec2<usize>> = HashSet::new();
        let outline_alpha = self.outline_color.value >> 24;

        if outline_alpha != 0xFF {
            let r_f32 = self.radius as f32;

            let sample_points = 4f32 * PI * r_f32 * r_f32.sqrt(); // Maybe try not to use sqrt, but for now it's to make scaling consistent, also it can grow pretty quickly
            let mut i = 0f32;

            loop {
                let px_x = (i.cos() + 1f32) * r_f32;
                let px_y = (i.sin() + 1f32) * r_f32;

                if px_x >= 0f32 && px_y >= 0f32 {
                    outline_points.insert(Vec2::new(px_x as usize, px_y as usize));
                }

                if i >= sample_points {
                    break;
                }
                i += 1f32;
            }
        }

        // Check whether color is fully transparent, if so, don't draw fill
        if self.fill_color.value >> 24 != 0xFF {  // 0xFF000000 -> 0x000000FF (6 hex sh = 24 bin sh)
            let (start_offset, end_offset) = if outline_alpha == 0xFF {
                (-(self.radius as isize), self.radius as isize)
            } else {
                (-(self.radius as isize) + 1, (self.radius as isize) - 1)
            };

            for x_px in start_offset..end_offset {
                for y_px in start_offset..end_offset {
                    if x_px * x_px + y_px * y_px <= (self.radius * self.radius) as isize {
                        let x_fin = (x_px + self.radius as isize) as usize;
                        let y_fin = (y_px + self.radius as isize) as usize;
                        if !outline_points.contains(&Vec2::new(x_fin, y_fin)) {
                            graphics_2d._set_pixel(
                                x_fin,
                                y_fin,
                                &self.fill_color,
                                &self.base
                            );
                        }
                    }
                }
            }
        }

        // Draw outline (if present)
        if !outline_points.is_empty() {
            for point in outline_points {
                graphics_2d._set_pixel(point.x, point.y, &self.outline_color, &self.base);
            }
        }
    }
}
