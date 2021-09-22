use crate::prelude::*;
const NUM_TILES:usize = (ARENA_HEIGHT * ARENA_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn map_idx(x: usize, y: usize) -> usize {
        ((y * ARENA_WIDTH as usize) + x) as usize
    }
    
    pub fn map_idx2point(idx: usize) -> Point {
        Point::new(idx % ARENA_WIDTH, idx / ARENA_WIDTH)
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        point.x >= 0 && point.x < ARENA_WIDTH as i32 && point.y >= 0 && point.y < ARENA_HEIGHT as i32
    }
}