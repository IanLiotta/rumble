use crate::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

#[system]
#[read_component(WantsToSpawn)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Spawner)]
#[read_component(Enemy)]
#[read_component(Render)]
pub fn spawn_mob(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] turn_queue: &mut TurnQueue,
    commands: &mut CommandBuffer,
) {
    let mut rng = rand::thread_rng();
    // check how many mobs currently exist
    let mobs: Vec<Point> = <&Point>::query()
        .filter(component::<Enemy>() | component::<Player>())
        .iter(ecs)
        .map(|mob| *mob)
        .collect();
    // if there are fewer than four mobs, roll to queue up another one to spawn
    if mobs.len() < 4 && rng.gen::<f32>() <= 0.03 {
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
                        let player_entity = commands.push((
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
                            Score { current: 0, max: 0 },
                        ));
                        //put the new player in the turn queue
                        turn_queue.queue.push_back(player_entity);
                        // make and attach a gun to the new player
                        commands.push((
                            Weapon {
                                name: "Laser".to_string(),
                                range: 6.5,
                            },
                            WeaponEquipped {
                                owner: player_entity,
                            },
                            WeaponDamageDirect { damage: 3 },
                            WeaponUsesEnergy { amount: 5 },
                        ));
                        //otherwise make an enemy
                    } else {
                        turn_queue.queue.push_back(commands.push((
                            Enemy { value: 1000 },
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
