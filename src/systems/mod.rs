use crate::prelude::*;

mod render_map;
mod render_entity;
mod movement;
mod player_input;
mod spawner;
mod end_turn;
mod random_walk;
mod render_hud;
mod damage;
mod targeting;
mod map_indexer;

pub fn build_round_start_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(spawner::spawn_mob_system())
    .add_system(map_indexer::map_indexer_system())
    .add_system(end_turn::end_turn_system())
    .build()
}

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(render_map::render_map_system())
    .add_system(render_entity::render_entity_system())
    .add_system(render_hud::render_hud_system())
    .build()
}

pub fn build_targeting_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(targeting::targeting_system())
    .flush()
    .add_system(render_map::render_map_system())
    .add_system(render_entity::render_entity_system())
    .add_system(render_hud::render_hud_system())
    .add_system(damage::damage_system())
    .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(render_map::render_map_system())
        .add_system(render_entity::render_entity_system())
        .add_system(render_hud::render_hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(spawner::spawn_mob_system())
    .flush()
    .add_system(random_walk::random_walk_system())
    .flush()
    .add_system(movement::move_entity_system())
    .flush()
    .add_system(render_map::render_map_system())
    .add_system(render_entity::render_entity_system())
    .add_system(render_hud::render_hud_system())
    .add_system(map_indexer::map_indexer_system())
    .add_system(end_turn::end_turn_system())
    .build()
}