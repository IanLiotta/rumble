use crate::prelude::*;

mod chasing;
mod damage;
mod end_turn;
mod enemy_attack;
mod fov;
mod map_indexer;
mod movement;
mod player_input;
mod random_walk;
mod render_entity;
mod render_hud;
mod render_map;
mod spawner;
mod targeting;

pub fn build_round_start_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(spawner::spawn_mob_system())
        .flush()
        .add_system(map_indexer::map_indexer_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .add_system(render_map::render_map_system())
        .add_system(render_entity::render_entity_system())
        .add_system(render_hud::render_hud_system())
        .add_system(player_input::player_input_system())
        .build()
}

pub fn build_targeting_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .add_system(map_indexer::map_indexer_system())
        .flush()
        .add_system(render_map::render_map_system())
        .add_system(render_entity::render_entity_system())
        .add_system(render_hud::render_hud_system())
        .add_system(targeting::targeting_system())
        .flush()
        .add_system(damage::damage_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .add_system(render_map::render_map_system())
        .add_system(render_entity::render_entity_system())
        .add_system(render_hud::render_hud_system())
        .add_system(movement::move_entity_system())
        .flush()
        .add_system(map_indexer::map_indexer_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(spawner::spawn_mob_system())
        .flush()
        //.add_system(random_walk::random_walk_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(movement::move_entity_system())
        .flush()
        .add_system(fov::fov_system())
        //.add_system(enemy_attack::enemy_attack_system())
        .add_system(render_map::render_map_system())
        .add_system(render_entity::render_entity_system())
        .add_system(render_hud::render_hud_system())
        .add_system(map_indexer::map_indexer_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}
