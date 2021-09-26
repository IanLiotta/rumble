use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToSpawn)]
#[read_component(Player)]
pub fn spawn_mob(
    player: Option<&Player>,
    entity: &Entity,
    _want_spawn: &WantsToSpawn,
    #[resource] map: &Map,
    commands: &mut CommandBuffer,
) {
    let mut rng = RandomNumberGenerator::new();
    let mut mob_placed = false;
    while !mob_placed {
        let loc = Map::map_idx(
            rng.range(1, ARENA_WIDTH - 1),
            rng.range(1, ARENA_HEIGHT - 1),
        );
        if map.tiles[loc] == TileType::Floor {
            if let Some(_player) = player {
                commands.push((
                    Player,
                    Map::map_idx2point(loc),
                    Render {
                        color: ColorPair::new(BLUE, BLACK),
                        glyph: to_cp437('@'),
                    },
                    DrawOffset {
                        offset_x: 0.0,
                        offset_y: 0.0,
                    },
                    MovementRange {
                        move_range: Vec::new(),
                    },
                    Health { hp: 32 },
                    FieldOfView::new(50),
                ));
            } else {
                commands.push((
                    Enemy,
                    Map::map_idx2point(loc),
                    Render {
                        color: ColorPair::new(YELLOW, BLACK),
                        glyph: 3,
                    },
                    DrawOffset {
                        offset_x: 0.0,
                        offset_y: 0.0,
                    },
                    MovementRange {
                        move_range: Vec::new(),
                    },
                    Health { hp: 3 },
                    MovesRandomly,
                    FieldOfView::new(50),
                ));
            }

            mob_placed = true;
        }
    }
    commands.remove(*entity);
}
