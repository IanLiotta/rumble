use crate::prelude::*;
use std::collections::HashSet;

// helper function to find tiles within a certain distance of an origin point
pub fn tiles_in_range(map: &Map, range: f32, origin: usize) -> HashSet<usize> {
    let mut result = HashSet::new();
    is_tile_in_range(map, range, origin, origin, &mut result);
    result
}

fn is_tile_in_range(map: &Map, range: f32, origin: usize, cell: usize, solution: &mut HashSet<usize>) {
    solution.insert(cell);
    for n in Map::get_neighbors(map, cell) {
        if !solution.contains(&n) && DistanceAlg::Pythagoras.distance2d(Map::map_idx2point(n), Map::map_idx2point(origin)) <= range {
            is_tile_in_range(map, range, origin, n, solution);
        }
    }
}