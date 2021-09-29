use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasesPlayer)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(IsMoving)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(
        Entity,
        &Point,
        &ChasesPlayer,
        &FieldOfView,
        Option<&IsMoving>,
    )>::query();
    //let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = Map::map_idx(player_pos.x as usize, player_pos.y as usize);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(ARENA_WIDTH, ARENA_HEIGHT, &search_targets, map, 1024.0);
    movers
        .iter(ecs)
        .for_each(|(entity, pos, _, fov, is_moving)| {
            // If this enemy is already on the move, don't continue
            if let Some(_is_moving) = is_moving {
                return;
            }
            let idx = Map::map_idx(pos.x as usize, pos.y as usize);
            let mut possible_moves: Vec<usize> = vec![];
            tiles_in_range(map, 3.0, idx).iter().for_each(|tile| {
                //only try to move onto visible, empty tiles
                if fov.visible_tiles.contains(&Map::map_idx2point(*tile))
                    && map.tile_contents[*tile].is_empty()
                {
                    possible_moves.push(*tile);
                }
            });
            // queue command to add the MovementRange component to the enemy entity
            commands.add_component(
                *entity,
                MovementRange {
                    move_range: possible_moves.clone(),
                },
            );
            let mut dest_found = false;
            let mut current_step = idx;
            let mut destination = 0;
            while !dest_found {
                if let Some(step) = DijkstraMap::find_lowest_exit(&dijkstra_map, current_step, map)
                {
                    let distance =
                        DistanceAlg::Pythagoras.distance2d(Map::map_idx2point(step), *player_pos);
                    if distance > 1.4 && possible_moves.contains(&step) {
                        destination = step;
                        current_step = step;
                    } else {
                        dest_found = true;
                        commands.push((
                            (),
                            WantsToMove {
                                entity: *entity,
                                source: Map::map_idx2point(idx),
                                destination: Map::map_idx2point(destination),
                            },
                        ));
                    }
                }
            }
        });
}
