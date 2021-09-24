use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(MovementRange)]
pub fn render_map(ecs: &SubWorld, #[resource] map: &Map, #[resource] turn_state: &TurnState, #[resource]spritesheet: &Texture2D) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    // draw normal map tiles
    for idx in 0..map.tiles.len() {
        let tile_point = Map::map_idx2point(idx);
        let spr_idx = match map.tiles[idx] {
            TileType::Floor => 0,
            TileType::Wall => 1,
        };
        draw_sprite(spritesheet, spr_idx, tile_point.x as f32, tile_point.y as f32)
    }
    /*
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
                        to_cp437('.')
                    );
                }
            });
        }
        _ => {}
    }
    */
}