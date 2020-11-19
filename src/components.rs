pub struct Piece;

pub struct Rotation(pub usize);

pub struct BlocPosition(pub usize);
pub struct Active;

pub struct Collider;

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

impl GridPos {
    pub fn x_pixels(&self, width: f32, origin: f32) -> f32 {
        (self.x as f32 + 0.5) * width + origin
    }
    pub fn y_pixels(&self, height: f32, origin: f32) -> f32 {
        -(self.y as f32 + 0.5) * height + origin
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Movement {
    None,
    Left,
    Right,
    Rotation,
    Down,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn grid_to_pixels() {
        let g = GridPos { x: 0, y: 0 };
        assert_eq!(g.x_pixels(50.0, -250.0), -225.0)
    }
}
