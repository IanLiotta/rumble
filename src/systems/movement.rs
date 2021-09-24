use crate::prelude::*;

// Consumes WantsToMove message-entities and attempts the move
// Also the place for synching the camera eventually
#[system(for_each)]
#[read_component(Player)]
pub fn move_entity(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        let offset_x = (want_move.source.x - want_move.destination.x) as f32;
        let offset_y = (want_move.source.y - want_move.destination.y) as f32;
        commands.add_component(want_move.entity, want_move.destination);
        commands.add_component(want_move.entity, DrawOffset{offset_x, offset_y});
    }
    commands.remove(*entity);
}