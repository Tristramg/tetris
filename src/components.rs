pub struct Piece;

pub struct Rotation(pub usize);

pub struct BlocPosition(pub usize);
pub struct Active;

pub struct Collider;

#[derive(Debug)]
pub struct Blocked {
    pub left: bool,
    pub right: bool,
}

pub struct GridPos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Movement {
    None,
    Left,
    Right,
    Rotation,
    Down,
}
