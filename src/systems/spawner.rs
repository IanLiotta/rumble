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
    let mobs: Vec<Point> = <&Point>::query()
        .filter(component::<Enemy>() | component::<Player>())
        .iter(ecs)
        .map(|mob| *mob)
        .collect();
    if mobs.len() < 4 {
        commands.push(((), WantsToSpawn));
    }
    if let Some(spawn_request) = <(Entity, &WantsToSpawn, Option<&Player>)>::query()
        .iter(ecs)
        .nth(0)
    {
        let (entity, _request, player) = spawn_request;
        let mut mob_placed = false;
        let mut spawners: Vec<Point> = <&Point>::query()
            .filter(component::<Spawner>())
            .iter(ecs)
            .map(|point| *point)
            .collect();
        let mut rng = rand::thread_rng();
        spawners.shuffle(&mut rng);
        let mut spawners_iter = spawners.iter();
        while !mob_placed {
            if let Some(point) = spawners_iter.next() {
                let loc = Map::map_idx(point.x as usize, point.y as usize);
                if map.tiles[loc] == TileType::Floor && !mobs.contains(point) {
                    if let Some(_player) = player {
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
                    mob_placed = true;
                }
            } else {
                println!("Oops, couldn't find a spawner for someone!");
                break;
            }
        }
        commands.remove(*entity);
    }
}
