pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub spr_idx: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove{
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToSpawn;

#[derive(Clone, Debug, PartialEq)]
pub struct MovementRange {
    pub move_range: Vec<usize>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovesRandomly;