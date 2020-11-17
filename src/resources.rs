use bevy::prelude::*;

pub struct ControlTimer(pub Timer);
pub struct SpeedTimer(pub Timer);
pub struct Scoreboard {
    pub score: usize,
    pub game_over: bool,
}
