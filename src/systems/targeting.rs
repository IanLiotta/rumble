use crate::prelude::*;

//TODO: Replace this with a component value later
const PLAYER_TARGETING_RANGE: f32 = 5.0;
#[system]
#[read_component(WantsToAttack)]
#[read_component(Point)]
#[read_component(FieldOfView)]
pub fn targeting(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] input_events: &mut std::collections::VecDeque<BEvent>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    <(Entity, &WantsToAttack, &Point, &FieldOfView)>::query()
        .iter(ecs)
        .for_each(|(entity, _, pos, fov)| {
            let target_tiles = tiles_in_range(
                map,
                PLAYER_TARGETING_RANGE,
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
                            //*turn_state = TurnState::EnemyTurn;
                        } else {
                            commands.remove_component::<WantsToAttack>(*entity);
                            //*turn_state = TurnState::AwaitingInput;
                        }
                    }
                    _ => {}
                }
            }
        });
    draw_batch.submit(2200).expect("Batch error");
}
