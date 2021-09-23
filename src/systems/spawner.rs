use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToSpawn)]
#[read_component(Player)]
pub fn spawn_mob(player: Option<&Player>, entity: &Entity, _want_spawn: &WantsToSpawn, #[resource]map: &Map, commands: &mut CommandBuffer){
    let mut rng = RandomNumberGenerator::new();
    let mut mob_placed = false;
        while !mob_placed {
            let loc = Map::map_idx(rng.range(1, ARENA_WIDTH - 1), rng.range(1, ARENA_HEIGHT - 1));
            if map.tiles[loc] == TileType::Floor {
               if let Some(_player) = player {
                commands.push(
                    (
                    Player,
                    Map::map_idx2point(loc),
                    Render{
                        spr_idx: 2
                    },
                    MovementRange {move_range: Vec::new()}
                    )
                );
                } else {
                    commands.push(
                        (
                        Enemy,
                        Map::map_idx2point(loc),
                        Render{
                            spr_idx: 3,
                        },
                        MovementRange {move_range: Vec::new()}
                        )
                    );
                }
                 
                mob_placed = true;
            }
        }
        commands.remove(*entity);
   
}