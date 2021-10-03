use crate::prelude::*;
use rand::seq::SliceRandom;

#[system]
#[read_component(WantsToSpawn)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Spawner)]
#[read_component(Enemy)]
pub fn spawn_mob(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] turn_queue: &mut TurnQueue,
    commands: &mut CommandBuffer,
) {
    // check how many mobs currently exist
    let mobs: Vec<Point> = <&Point>::query()
        .filter(component::<Enemy>() | component::<Player>())
        .iter(ecs)
        .map(|mob| *mob)
        .collect();
    // if there are fewer than four mobs, queue up another one to spawn
    if mobs.len() < 4 {
        commands.push(((), WantsToSpawn));
    }
    // Check if there are queued spawn requests (and if they're for the player)
    if let Some(spawn_request) = <(Entity, &WantsToSpawn, Option<&Player>)>::query()
        .iter(ecs)
        .nth(0)
    {
        // destructure the spawn request
        let (entity, _request, player) = spawn_request;
        let mut mob_placed = false;
        // get the list of spawners and randomize their order
        let mut spawners: Vec<Point> = <&Point>::query()
            .filter(component::<Spawner>())
            .iter(ecs)
            .map(|point| *point)
            .collect();
        let mut rng = rand::thread_rng();
        spawners.shuffle(&mut rng);
        let mut spawners_iter = spawners.iter();
        // loop while we try to place the mob at each spawner
        while !mob_placed {
            // check if any spawners are left in the list
            if let Some(point) = spawners_iter.next() {
                // get the spawner location
                let loc = Map::map_idx(point.x as usize, point.y as usize);
                // make sure the spawn point doesn't already contain a mob
                if map.tiles[loc] == TileType::Floor && !mobs.contains(point) {
                    // if there was a player spawn request,
                    if let Some(_player) = player {
                        // check if the player already exists - move it if so
                        let existing_player = <(Entity, &Player, &Point)>::query().iter(ecs).nth(0);
                        if let Some(existing_player) = existing_player {
                            commands.add_component(*existing_player.0, Map::map_idx2point(loc));
                        // otherwise make a new one
                        } else {
                            turn_queue.queue.push_back(commands.push((
                                Player,
                                Map::map_idx2point(loc),
                                Render {
                                    color: ColorPair::new(BLUE, BLACK),
                                    glyph: to_cp437('@'),
                                },
                                MovementRange {
                                    move_range: Vec::new(),
                                },
                                Health { hp: 32, max_hp: 32 },
                                Energy {
                                    energy: 100,
                                    max_energy: 100,
                                },
                                FieldOfView::new(50),
                            )));
                        } // spawend or moved player
                          //otherwise make an enemy
                    } else {
                        turn_queue.queue.push_back(commands.push((
                            Enemy,
                            Map::map_idx2point(loc),
                            Render {
                                color: ColorPair::new(YELLOW, BLACK),
                                glyph: 3,
                            },
                            MovementRange {
                                move_range: Vec::new(),
                            },
                            Health { hp: 3, max_hp: 3 },
                            Energy {
                                energy: 100,
                                max_energy: 100,
                            },
                            ChasesPlayer,
                            FieldOfView::new(50),
                        )));
                    }
                    // signal that we spawned the mob so the loop ends
                    mob_placed = true;
                }
            }
        } // close while !mob_placed
          // remove the spawn request
        commands.remove(*entity);
    } // close spawn request check
} // close fn
