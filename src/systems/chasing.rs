use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasesPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasesPlayer)>::query();
    //let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = Map::map_idx(player_pos.x as usize, player_pos.y as usize);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(ARENA_WIDTH, ARENA_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let idx = Map::map_idx(pos.x as usize, pos.y as usize);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            if distance > 1.2 {
                let destination_pt = Map::map_idx2point(destination);
                if map.tile_contents[destination].is_empty() {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            source: *pos,
                            destination: destination_pt,
                        },
                    ));
                }
            }
        }
    });
}
