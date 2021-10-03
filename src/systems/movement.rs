use crate::prelude::*;

// 1: Entity issues a WantsToMove message, either from player input or AI, and it's recieved here
// 2: The WantToMove message is resolved and if valid generates an IsMoving message with the path
// 3: If any IsMoving messages are outstanding, update entity position one step. Remove if finished.

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(WantsToMove)]
#[write_component(IsMoving)]
pub fn move_entity(ecs: &mut SubWorld, #[resource] map: &Map, commands: &mut CommandBuffer) {
    // 1: Process and delete move requests.
    let mut move_requests = <(Entity, &WantsToMove)>::query();
    for (entity, want_move) in move_requests.iter(ecs) {
        // Check if it's a valid move
        if map.can_enter_tile(want_move.destination) {
            let path = a_star_search(
                Map::map_idx(want_move.source.x as usize, want_move.source.y as usize),
                Map::map_idx(
                    want_move.destination.x as usize,
                    want_move.destination.y as usize,
                ),
                map,
            );
            // 2: issue the command to actually move
            commands.add_component(want_move.entity, IsMoving { path });
        }
        // remove the request to move
        commands.remove(*entity);
    }

    // 3: Check if there are any outstanding movers and advance them one step.
    let mut movers = <(Entity, &mut IsMoving, &FieldOfView)>::query();
    for (entity, is_moving, mover_fov) in movers.iter_mut(ecs) {
        let next_step = Map::map_idx2point(is_moving.path.steps.remove(0));
        commands.add_component(*entity, next_step);
        if is_moving.path.steps.is_empty() {
            commands.remove_component::<IsMoving>(*entity);
        }
        commands.add_component(*entity, mover_fov.clone_dirty());
    }
}
