pub struct BlocPosition(pub usize);
pub struct Active;

#[derive(Debug)]
pub struct Blocked {
    pub left: bool,
    pub right: bool,
    pub bottom: bool,
}

#[derive(Debug)]
pub struct GridPos {
    pub x: isize,
    pub y: isize,
}
