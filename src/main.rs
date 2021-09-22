mod map;
mod systems;
mod map_builder;
mod components;
mod spawner;
mod turn_state;
mod movement_range;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::map_builder::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::turn_state::*;
    pub use crate::movement_range::*;
    pub const SCREEN_HEIGHT:i32 = 50;
    pub const SCREEN_WIDTH:i32 = 80;
    pub const ARENA_HEIGHT:usize = 40;
    pub const ARENA_WIDTH:usize = 40;
    pub const FRAME_DURATION:f32 = 60.0;
}

use prelude::*;

enum GameMode {
    Playing,
    Shop,
}

struct State {
    ecs: World,
    resources: Resources,
    game_mode: GameMode,
    frame_time: f32,
    player_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mb = MapBuilder::new();
        resources.insert(mb.map);
        spawn_player(&mut world, Map::map_idx2point(mb.player_start));
        resources.insert(TurnState::AwaitingInput);
        State {
            ecs: world,
            resources: resources,
            game_mode: GameMode::Playing,
            frame_time: 0.0,
            player_systems: build_player_scheduler(),
        }
    }
}

impl GameState for State {
    // run by main_loop
    fn tick(&mut self, ctx: &mut BTerm) {
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => {self.player_systems.execute(&mut self.ecs, &mut self.resources);},
            _ => {}
        }
        //draw the buffer constructed in multiple places elsewhere
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_resource_path("resources/")
        .with_font("terminal8x8.png", 8, 8)
        .with_tile_dimensions(16, 16)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_title("Rumble")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .build()?;
    let gs = State::new();
    main_loop(context, gs)
}
