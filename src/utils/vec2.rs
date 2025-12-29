#[derive(Default)]
pub(crate) struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}