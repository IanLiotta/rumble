// Accept mouse input to move player
// Also discover the movement range of the player
// and add it as a component to the player entity

use crate::prelude::*;
//replace this with a component value later
const PLAYER_MOVE_RANGE: f32 = 4.0;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(FieldOfView)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] turn_queue: &TurnQueue,
    #[resource] input_events: &mut std::collections::VecDeque<BEvent>,
    commands: &mut CommandBuffer,
) {
    let input = INPUT.lock();
    let mut mobs = <(Entity, &Point)>::query().filter(component::<Enemy>());
    let mut mobs_idx = Vec::new();
    mobs.iter(ecs).for_each(|(_, mob_point)| {
        mobs_idx.push(Map::map_idx(mob_point.x as usize, mob_point.y as usize));
    });
    let mouse_pos = input.mouse_tile(0);
    let mouse_idx = Map::map_idx(mouse_pos.x as usize, mouse_pos.y as usize);

    // find each player controlled entity
    let player_entity = turn_queue.queue[0];
    let player_entry = ecs.entry_ref(player_entity).unwrap();
    let player_pos = player_entry.get_component::<Point>().unwrap();
    let player_fov = player_entry.get_component::<FieldOfView>().unwrap();
    // find the valid moves within the player's movement range
    let mut possible_moves: Vec<usize> = vec![];
    tiles_in_range(
        map,
        PLAYER_MOVE_RANGE,
        Map::map_idx(player_pos.x as usize, player_pos.y as usize),
    )
    .iter()
    .for_each(|tile| {
        if player_fov
            .visible_tiles
            .contains(&Map::map_idx2point(*tile))
        {
            possible_moves.push(*tile);
        }
    });
    // Draw the mouse cursor
    if possible_moves.contains(&mouse_idx) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);
        draw_batch.set(
            mouse_pos,
            ColorPair::new(RGBA::from_f32(0.0, 1.0, 0.0, 0.8), BLACK),
            to_cp437('X'),
        );
        draw_batch.submit(2200).expect("Batch error");
    }

    // queue command to add the MovementRange component to the player entity
    commands.add_component(
        player_entity,
        MovementRange {
            move_range: possible_moves.clone(),
        },
    );
    // if a valid tile is clicked, queue a message-entity that the player wants to move
    while let Some(event) = input_events.pop_front() {
        match event {
            BEvent::MouseButtonDown { button: 0 } => {
                if !mobs_idx.contains(&mouse_idx) && possible_moves.contains(&mouse_idx) {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: player_entity,
                            source: *player_pos,
                            destination: mouse_pos,
                        },
                    ));
                }
            }
            BEvent::KeyboardInput {
                key: VirtualKeyCode::Key1,
                pressed: true,
                ..
            } => {
                commands.add_component(player_entity, WantsToAttack {});
            }
            _ => {}
        }
    }
}
