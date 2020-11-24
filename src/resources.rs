use bevy::prelude::*;

pub struct ControlTimer(pub Timer);
pub struct SpeedTimer(pub Timer);

pub struct Grid {
    pub unit: f32,
    pub width: isize,
    pub height: isize,
}

impl Grid {
    pub fn x_min(&self) -> f32 {
        self.unit * self.width as f32 * -0.5
    }

    pub fn x_max(&self) -> f32 {
        -self.x_min()
    }

    pub fn width(&self) -> f32 {
        self.x_max() - self.x_min()
    }

    pub fn y_min(&self) -> f32 {
        self.unit * self.height as f32 * 0.5
    }

    pub fn y_max(&self) -> f32 {
        -self.y_min()
    }

    pub fn height(&self) -> f32 {
        self.y_min() - self.y_max()
    }

    pub fn as_translation(&self, x: isize, y: isize) -> Vec3 {
        Vec3::new(
            (x as f32 + 0.5) * self.unit + self.x_min(),
            -(y as f32 + 0.5) * self.unit + self.y_min(),
            0.0,
        )
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

pub struct Status {
    pub blocked_left: bool,
    pub blocked_right: bool,
    pub blocked_bottom: bool,
    pub next_movement: Movement,
    pub rotation: usize,
    pub x: isize,
    pub y: isize,
    pub piece: crate::constants::Tetromino,
    pub score: usize,
    pub game_over: bool,
}
