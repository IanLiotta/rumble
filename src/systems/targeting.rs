use crate::prelude::*;

//TODO: Replace this with a component value later
const PLAYER_TARGETING_RANGE: f32 = 4.5;
#[system]
#[read_component(WantsToAttack)]
#[read_component(Point)]
#[read_component(FieldOfView)]
#[read_component(Weapon)]
pub fn targeting(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] input_events: &mut std::collections::VecDeque<BEvent>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    // get each entity that wants to attack, its position, and fov.
    <(Entity, &WantsToAttack, &Point, &FieldOfView)>::query()
        .iter(ecs)
        .for_each(|(entity, attack_req, pos, fov)| {
            // get the weapon being used to attack
            let weapon = ecs.entry_ref(attack_req.weapon).unwrap();
            let weapon_range = weapon.get_component::<Weapon>().unwrap().range;
            // get and draw targeting range
            let target_tiles = tiles_in_range(
                map,
                weapon_range,
                Map::map_idx(pos.x as usize, pos.y as usize),
            )
            .into_iter()
            .filter(|tile| fov.visible_tiles.contains(&Map::map_idx2point(*tile)))
            .collect::<Vec<usize>>();
            target_tiles.iter().for_each(|target| {
                draw_batch.set(
                    Map::map_idx2point(*target),
                    ColorPair::new(RGBA::from_f32(1.0, 0.0, 0.0, 0.5), BLACK),
                    178,
                );
            });
            let input = INPUT.lock();
            let mouse_pos = input.mouse_tile(0);
            draw_batch.set(
                mouse_pos,
                ColorPair::new(RGBA::from_f32(1.0, 0.0, 0.0, 0.8), BLACK),
                to_cp437('X'),
            );
            let mouse_idx = Map::map_idx(mouse_pos.x as usize, mouse_pos.y as usize);
            while let Some(event) = input_events.pop_front() {
                match event {
                    BEvent::MouseButtonDown { button: 0 } => {
                        if target_tiles.contains(&mouse_idx) {
                            //Find if there's anything in the target tile and tag it to take damage
                            let entities = &map.tile_contents[mouse_idx];
                            for entity in entities.iter() {
                                commands.push((
                                    (),
                                    DrawLine {
                                        source: *pos,
                                        dest: mouse_pos,
                                        duration: 10,
                                    },
                                ));
                                commands.add_component(
                                    *entity,
                                    DirectDamage {
                                        amount: 10,
                                        source: *pos,
                                    },
                                );
                            }
                            commands.remove_component::<WantsToAttack>(*entity);
                        } else {
                            commands.remove_component::<WantsToAttack>(*entity);
                        }
                    }
                    _ => {}
                }
            }
        });
    draw_batch.submit(2200).expect("Batch error");
}
