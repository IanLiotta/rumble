use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Enemy)]
#[read_component(FieldOfView)]
pub fn enemy_attack(ecs: &SubWorld, #[resource] map: &Map, commands: &mut CommandBuffer) {
    <(Entity, &Point, &FieldOfView)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .for_each(|(entity, pos, fov)| {
            // get a list of other entities inside FOV
            // if there's more than one, roll to pick a random one
            // blat blat
            let target_tiles =
                tiles_in_range(map, 5.0, Map::map_idx(pos.x as usize, pos.y as usize))
                    .into_iter()
                    .filter(|tile| fov.visible_tiles.contains(&Map::map_idx2point(*tile)))
                    .collect::<Vec<usize>>();
            let targets: Vec<&usize> = target_tiles
                .iter()
                .filter(|tile| !map.tile_contents[**tile].is_empty())
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
            }
        });
}
