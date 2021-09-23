mod map;
mod systems;
mod map_builder;
mod components;
mod turn_state;
mod movement_range;
mod drawing;

mod prelude {
    pub use macroquad::prelude::*;
    pub use bracket_lib::prelude::*;
    pub use bracket_lib::prelude::{Rect};
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::map_builder::*;
    pub use crate::components::*;
    pub use crate::turn_state::*;
    pub use crate::movement_range::*;
    pub use crate::drawing::*;
    pub const SCREEN_HEIGHT:i32 = 50;
    pub const SCREEN_WIDTH:i32 = 80;
    pub const ARENA_HEIGHT:usize = 40;
    pub const ARENA_WIDTH:usize = 40;
    pub const FRAME_DURATION:f32 = 60.0; // holding on to this - probably good to limit cpu usage?
    pub const SPRITESHEET_HEIGHT:i32 = 8;
    pub const SPRITESHEET_WIDTH:i32 = 8;
    pub const TILE_SIZE:f32 = 32.;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    frame_time: f32,
    round_start_systems: Schedule,
    input_systems: Schedule,
    player_systems: Schedule,
    enemy_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let world = World::default();
        let mut resources = Resources::default();
        let mb = MapBuilder::new();
        let spritesheet = Texture2D::from_file_with_format(
            include_bytes!("../resources/spritesheet.png"),
            None,
        );
        resources.insert(mb.map);
        resources.insert(TurnState::StartGame);
        resources.insert(spritesheet);
        State {
            ecs: world,
            resources: resources,
            frame_time: 0.0,
            round_start_systems: build_round_start_scheduler(),
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            enemy_systems: build_enemy_scheduler(),
        }
    }
}

#[macroquad::main("rumble")]
async fn main() {
    let mut state = State::new();
    loop {
        clear_background(macroquad::color::BLACK);
        let current_state = state.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::StartGame => {
                // Move these mob creations into the round start system eventually
                state.ecs.push((Player, WantsToSpawn));
                state.ecs.push(((), WantsToSpawn));
                state.ecs.push(((), WantsToSpawn));
                state.round_start_systems.execute(&mut state.ecs, &mut state.resources);
            }
            TurnState::AwaitingInput => {state.input_systems.execute(&mut state.ecs, &mut state.resources);},
            TurnState::PlayerTurn => {state.player_systems.execute(&mut state.ecs, &mut state.resources);},
            TurnState::EnemyTurn => {state.enemy_systems.execute(&mut state.ecs, &mut state.resources);},
        }
        next_frame().await
    }
}
