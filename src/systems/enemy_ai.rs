use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Enemy)]
#[read_component(FieldOfView)]
#[read_component(IsMoving)]
#[read_component(ChasesPlayer)]
#[read_component(Player)]
pub fn enemy_ai(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    // Get the list of enemies
    // If the enemy IsMoving, do nothing - movement system will take care of it
    // Otherwise, look for another entity. If they're in your attack range, attack, otherwise move
    let mut enemies =
        <(Entity, &Point, Option<&IsMoving>, &FieldOfView)>::query().filter(component::<Enemy>());
    enemies.iter(ecs).for_each(|(entity, pos, is_moving, fov)| {
        if let Some(_is_moving) = is_moving {
            return;
        } else {
            if enemy_attack(ecs, map, commands, entity, pos, fov) {
                return;
            } else {
                chasing(ecs, map, commands);
            }
        }
    });
}

fn chasing(ecs: &SubWorld, map: &Map, commands: &mut CommandBuffer) {
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

//this should be part of a component
const ENEMY_SIGHT_RADIUS: f32 = 5.0;

pub fn enemy_attack(
    ecs: &SubWorld,
    map: &Map,
    commands: &mut CommandBuffer,
    entity: &Entity,
    pos: &Point,
    fov: &FieldOfView,
) -> bool {
    // get a list of other entities inside FOV
    // if there's more than one, roll to pick a random one
    // blat blat
    let target_tiles = tiles_in_range(
        map,
        ENEMY_SIGHT_RADIUS,
        Map::map_idx(pos.x as usize, pos.y as usize),
    )
    .into_iter()
    .filter(|tile| fov.visible_tiles.contains(&Map::map_idx2point(*tile)))
    .collect::<Vec<usize>>();
    let targets: Vec<&usize> = target_tiles
        .iter()
        .filter(|tile| {
            !map.tile_contents[**tile].is_empty()
                && **tile != Map::map_idx(pos.x as usize, pos.y as usize)
            // don't shoot yourself
        })
        .collect();
    if targets.len() >= 1 {
        let mut rng = RandomNumberGenerator::new();
        let target = targets[rng.roll_dice(0, (targets.len() - 1) as i32) as usize];
        commands.push((
            (),
            DrawLine {
                source: *pos,
                dest: Map::map_idx2point(*target),
                duration: 10,
            },
        ));
        commands.add_component(
            map.tile_contents[*target][0],
            DirectDamage {
                amount: 1,
                source: *pos,
            },
        );
        return true;
    }
    false
}
