use crate::prelude::*;



/*
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            MovementRange {move_range: Vec::new()},
            Render{
                color: ColorPair::new(RED, BLACK),
                glyph: to_cp437('@'),
            }
        )
    );
}
*/

#[system(for_each)]
#[read_component(WantsToSpawn)]
#[read_component(Player)]
pub fn spawn_mob(ecs: &mut SubWorld, player: Option<&Player>, entity: &Entity, _want_spawn: &WantsToSpawn, #[resource]map: &Map, commands: &mut CommandBuffer){
    let mut rng = RandomNumberGenerator::new();
    let mut mob_placed = false;
        while !mob_placed {
            let loc = Map::map_idx(rng.range(1, ARENA_WIDTH - 1), rng.range(1, ARENA_HEIGHT - 1));
            if map.tiles[loc] == TileType::Floor {
               if let Some(player) = player {
                commands.push(
                    (
                    Player,
                    Map::map_idx2point(loc),
                    Render{
                        color: ColorPair::new(BLUE,BLACK),
                        glyph: to_cp437('@'),
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
                            color: ColorPair::new(YELLOW,BLACK),
                            glyph: 3,
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