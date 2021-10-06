pub use crate::prelude::*;
use std::collections::HashSet;

pub struct Spawner {}

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

pub struct WantsToAttack {
    pub weapon: Entity,
}

pub struct WantsToLeave {}

pub struct WantsToPlay {}

pub struct DirectDamage {
    pub amount: i32,
    pub source: Point,
}

pub struct DrawLine {
    pub source: Point,
    pub dest: Point,
    pub duration: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToSpawn;

pub struct AddScore {
    pub score: i32,
}

// Mob traits

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy {
    pub value: i32,
}

pub struct Health {
    pub hp: i32,
    pub max_hp: i32,
}

pub struct Energy {
    pub energy: i32,
    pub max_energy: i32,
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

pub struct Score {
    pub current: i32,
    pub max: i32,
}

// Weapons
pub struct Weapon {
    pub name: String,
    pub range: f32,
}
pub struct WeaponEquipped {
    pub owner: Entity,
}

pub struct WeaponDamageDirect {
    pub damage: i32,
}

pub struct WeaponUsesEnergy {
    pub amount: i32,
}

pub struct WeaponUsesAmmo {
    pub amount: usize,
    pub current: usize,
    pub max: usize,
}
