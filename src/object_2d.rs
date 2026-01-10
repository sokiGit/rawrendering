use crate::utils::vec2::Vec2;

#[derive(Clone)]
pub struct Object2D {
    pub offset: Vec2<i32>,
    pub relative_offset: Vec2<f32>,
    pub anchor: Vec2<i32>,
    //TODO: pub relative_anchor: Vec2<f32>, (Will most likely require something like get_bounds_size() -> Vec2<usize>)
    //TODO: pub scale: f32
}

impl Default for Object2D {
    fn default() -> Self {
        Object2D {
            offset: Vec2::default(),
            relative_offset: Vec2::default(),
            anchor: Vec2::default(),
            //TODO: relative_anchor: Vec2::default(),
            //TODO: scale: 1f32,
        }
    }
}