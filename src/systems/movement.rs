use crate::prelude::*;

// Consumes WantsToMove message-entities and attempts the move
// Also the place for synching the camera eventually
#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn move_entity(
    ecs: &SubWorld,
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    commands: &mut CommandBuffer,
) {
    if let Ok(entry) = ecs.entry_ref(want_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(want_move.entity, fov.clone_dirty());
        }
    }
    if map.can_enter_tile(want_move.destination) {
        let offset_x = (want_move.source.x - want_move.destination.x) as f32;
        let offset_y = (want_move.source.y - want_move.destination.y) as f32;
        commands.add_component(want_move.entity, want_move.destination);
        commands.add_component(want_move.entity, DrawOffset{offset_x, offset_y});
    }
    commands.remove(*entity);
}