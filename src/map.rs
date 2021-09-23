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

    // gets striding index from x y coords
    pub fn map_idx(x: usize, y: usize) -> usize {
        ((y * ARENA_WIDTH as usize) + x) as usize
    }
    
    // turns striding index into a Point
    pub fn map_idx2point(idx: usize) -> Point {
        Point::new(idx % ARENA_WIDTH, idx / ARENA_WIDTH)
    }

    // returns true if the tile is in bounds and not a wall
    pub fn can_enter_tile(&self, point: Point) -> bool {
        point.x >= 0 && point.x < ARENA_WIDTH as i32 && point.y >= 0 && point.y < ARENA_HEIGHT as i32 && self.tiles[Map::map_idx(point.x as usize, point.y as usize)] == TileType::Floor
    }

    // get the neighboring cells that can be moved into
    pub fn get_neighbors(map: &Map, origin: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let deltas = [origin + 1, origin - 1, origin + ARENA_HEIGHT, origin - ARENA_HEIGHT];
        for d in deltas {
            if map.can_enter_tile(Map::map_idx2point(d)) {
                result.push(d);
            }
        }
        result
    }
}