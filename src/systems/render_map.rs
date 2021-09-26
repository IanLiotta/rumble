use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(MovementRange)]
#[read_component(FieldOfView)]
pub fn render_map(ecs: &SubWorld, #[resource] map: &Map, #[resource] turn_state: &TurnState) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    // draw normal map tiles
    for idx in 0..map.tiles.len() {
        if player_fov.visible_tiles.contains(&Map::map_idx2point(idx)) {
            let tile_point = Map::map_idx2point(idx);
            let glyph = match map.tiles[idx] {
                TileType::Floor => to_cp437('.'),
                TileType::Wall => to_cp437('#'),
            };
            draw_batch.set(tile_point, ColorPair::new(WHITE, BLACK), glyph);
        }
    }
    // draw player movement range
    match turn_state {
        TurnState::AwaitingInput => {
            // find entitites with MovementRange and filter for only player controlled ones
            let mut players = <(Entity, &MovementRange)>::query().filter(component::<Player>());
            players.iter(ecs).for_each(|(_, movement_range)| {
                for t in &movement_range.move_range {
                    draw_batch.set(
                        Map::map_idx2point(*t),
                        ColorPair::new(GREEN, BLACK),
                        to_cp437('.'),
                    );
                }
            });
        }
        _ => {}
    }
    draw_batch.submit(0).expect("Batch error");
}
