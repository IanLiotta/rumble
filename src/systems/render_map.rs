use crate::prelude::*;
use bracket_geometry::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn render_map(ecs: &SubWorld, #[resource] map: &Map, #[resource] turn_state: &TurnState) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    // draw normal map tiles
    for idx in 0..map.tiles.len() {
        let tile_point = Map::map_idx2point(idx);
        let glyph = match map.tiles[idx] {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
        };
        draw_batch.set(
            tile_point,
            ColorPair::new(WHITE, BLACK),
            glyph
        );
    }
    // draw player movement range
    match turn_state {
        TurnState::AwaitingInput => {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
            players.iter(ecs).for_each(|(_, player_point)| {
                let player_idx = Map::map_idx(player_point.x as usize, player_point.y as usize);
                let tiles_to_highlight = tiles_in_range(map, 4.0, player_idx);
                for t in tiles_to_highlight.iter() {
                    draw_batch.set(
                        Map::map_idx2point(*t),
                        ColorPair::new(GREEN, BLACK),
                        to_cp437('.')
                    );
                }
            });
        }
        _ => {}
    }
    draw_batch.submit(0).expect("Batch error");
}

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