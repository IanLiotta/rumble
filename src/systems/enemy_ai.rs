use crate::prelude::*;
use legion::world::*;

#[system]
#[read_component(Point)]
#[read_component(Enemy)]
#[read_component(FieldOfView)]
#[read_component(IsMoving)]
#[read_component(ChasesPlayer)]
#[read_component(Player)]
#[read_component(Health)]
pub fn enemy_ai(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] turn_queue: &TurnQueue,
) {
    // If the enemy IsMoving, do nothing - movement system will take care of it
    // Otherwise, look for another entity. If they're in your attack range, attack, otherwise move
    // Kludge to prevent ai from running when there's no enemies. TODO: new system to handle this?
    if turn_queue.queue.len() > 1 {
        let enemy_entity = turn_queue.queue[0];
        let enemy_entry = ecs.entry_ref(enemy_entity).unwrap();
        if let Ok(_is_moving) = enemy_entry.get_component::<IsMoving>() {
            return;
        } else {
            let pos = enemy_entry.get_component::<Point>().unwrap();
            let fov = enemy_entry.get_component::<FieldOfView>().unwrap();
            if enemy_attack(ecs, commands, pos, fov) {
                return;
            } else {
                chasing(ecs, enemy_entity, map, commands);
            }
        }
    }
}

fn chasing(ecs: &SubWorld, enemy: Entity, map: &Map, commands: &mut CommandBuffer) {
    // get the entry for our enemy
    let enemy_entry = ecs.entry_ref(enemy).unwrap();
    // If this enemy is already on the move, don't continue
    if let Ok(_is_moving) = enemy_entry.get_component::<IsMoving>() {
        return;
    }
    // find where the player is
    let mut player = <(&Point, &Player)>::query();
    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = Map::map_idx(player_pos.x as usize, player_pos.y as usize);
    // generate the dijkstra map
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(ARENA_WIDTH, ARENA_HEIGHT, &search_targets, map, 1024.0);
    // get the info about our enemy we'll need
    let pos = enemy_entry.get_component::<Point>().unwrap();
    let fov = enemy_entry.get_component::<FieldOfView>().unwrap();
    let idx = Map::map_idx(pos.x as usize, pos.y as usize);
    // figure out where the enemy is able to move to
    let mut possible_moves: Vec<usize> = vec![];
    tiles_in_range(map, 3.0, idx).iter().for_each(|tile| {
        //only try to move onto visible, empty tiles
        if fov.visible_tiles.contains(&Map::map_idx2point(*tile))
            && map.tile_contents[*tile].is_empty()
        {
            possible_moves.push(*tile);
        }
    });
    // queue command to add those possible moves to the enemy
    commands.add_component(
        enemy,
        MovementRange {
            move_range: possible_moves.clone(),
        },
    );
    // start searching for our path to the player
    let mut dest_found = false;
    let mut current_step = idx;
    let mut destination = 0;
    while !dest_found {
        if let Some(step) = DijkstraMap::find_lowest_exit(&dijkstra_map, current_step, map) {
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
                        entity: enemy,
                        source: Map::map_idx2point(idx),
                        destination: Map::map_idx2point(destination),
                    },
                ));
            }
        }
    }
}

//this should be part of a component
const ENEMY_SIGHT_RADIUS: f32 = 5.0;

pub fn enemy_attack(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    pos: &Point,
    fov: &FieldOfView,
) -> bool {
    // query for all enemies and players
    let mut targets =
        <(Entity, &Point, &Health)>::query().filter(component::<Enemy>() | component::<Player>());
    let visible_targets: Vec<(&Entity, &Point, &Health)> = targets
        .iter(ecs)
        .filter(|(_, point, _)| {
            fov.visible_tiles.contains(point)
                && DistanceAlg::Pythagoras.distance2d(*pos, **point) < ENEMY_SIGHT_RADIUS
                && *pos != **point
        })
        .collect();
    if visible_targets.len() >= 1 {
        let mut rng = RandomNumberGenerator::new();
        let target = visible_targets[rng.roll_dice(0, (visible_targets.len() - 1) as i32) as usize];
        commands.push((
            (),
            DrawLine {
                source: *pos,
                dest: *target.1,
                duration: 10,
            },
        ));
        commands.add_component(
            *target.0,
            DirectDamage {
                amount: 1,
                source: *pos,
            },
        );
        return true;
    }
    false
}
