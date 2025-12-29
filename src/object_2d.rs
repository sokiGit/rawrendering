use crate::utils::vec2::Vec2;
pub(crate) struct Object2D {
    pub(crate) offset: Vec2<isize>,
    pub(crate) relative_offset: Vec2<f32>,
    pub(crate) anchor: Vec2<isize>,
    //TODO: pub(crate) relative_anchor: Vec2<f32>, (Will most likely require something like get_bounds_size() -> Vec2<usize>)
    //TODO: pub(crate) scale: f32
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