use bevy::prelude::*;
pub struct Piece;

pub struct Rotation(pub usize);

pub struct BlocPosition(pub usize);
pub struct Active;
pub struct Velocity(pub Vec2);
pub struct Collider;

#[derive(Debug)]
pub struct Blocked {
    pub left: bool,
    pub right: bool,
}
