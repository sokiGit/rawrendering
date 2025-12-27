pub struct Polygon<'a> {
    pub path: &'a [(usize, usize)],
    pub color: u32,
    pub closed: bool,
}