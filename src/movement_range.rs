use crate::prelude::*;

pub fn tiles_in_range(map: &Map, range: f32, origin: usize) -> Vec<usize> {
    let mut result = Vec::new();
    is_tile_in_range(map, range, origin, origin, &mut result);
    result
}

fn is_tile_in_range(map: &Map, range: f32, origin: usize, cell: usize, solution: &mut Vec<usize>) {
    solution.push(cell);
    for n in Map::get_neighbors(map, cell) {
        if !solution.contains(&n) && DistanceAlg::Manhattan.distance2d(Map::map_idx2point(n), Map::map_idx2point(origin)) <= range {
            is_tile_in_range(map, range, origin, n, solution);
        }
    }
}