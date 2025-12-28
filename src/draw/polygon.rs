pub struct Polygon<'a> {
    pub path: &'a [(isize, isize)],
    pub color: u32,
    pub closed: bool,
}