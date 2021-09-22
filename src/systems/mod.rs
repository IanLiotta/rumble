use crate::prelude::*;

mod render_map;
mod render_entity;
mod movement;
mod player_input;

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(render_map::render_map_system())
        .add_system(render_entity::render_entity_system())
        .add_system(movement::move_entity_system())
        .add_system(player_input::player_input_system())
        .build()
}