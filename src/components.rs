pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

pub struct Health {
    pub hp: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove{
    pub entity: Entity,
    pub source: Point,
    pub destination: Point,
}

pub struct WantsToAttack {
    pub attacker: Entity,
    pub pos: Point,
}

pub struct DirectDamage {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToSpawn;

#[derive(Clone, Debug, PartialEq)]
pub struct MovementRange {
    pub move_range: Vec<usize>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovesRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawOffset {
    pub offset_x: f32,
    pub offset_y: f32,
}