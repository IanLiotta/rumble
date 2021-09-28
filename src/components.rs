pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

// Message Types

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub source: Point,
    pub destination: Point,
}

pub struct IsMoving {
    pub path: NavigationPath,
}

pub struct WantsToAttack {}

pub struct DirectDamage {
    pub amount: i32,
    pub source: Point,
}

pub struct DrawLine {
    pub source: Point,
    pub dest: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToSpawn;

// Mob traits

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

#[derive(Clone, Debug, PartialEq)]
pub struct MovementRange {
    pub move_range: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovesRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasesPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawOffset {
    pub offset_x: f32,
    pub offset_y: f32,
}
