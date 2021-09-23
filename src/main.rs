mod map;
mod systems;
mod map_builder;
mod components;
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
    pub use crate::turn_state::*;
    pub use crate::movement_range::*;
    pub const SCREEN_HEIGHT:i32 = 50;
    pub const SCREEN_WIDTH:i32 = 80;
    pub const ARENA_HEIGHT:usize = 40;
    pub const ARENA_WIDTH:usize = 40;
    pub const FRAME_DURATION:f32 = 60.0; // holding on to this - probably good to limit cpu usage?
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
        resources.insert(mb.map);
        resources.insert(TurnState::StartGame);
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

impl GameState for State {
    // run by main_loop
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::StartGame => {
                self.ecs.push((Player, WantsToSpawn));
                self.ecs.push(((), WantsToSpawn));
                self.ecs.push(((), WantsToSpawn));
                // possibly do this with current_state somehow instead of getting state again?
                //let mut change_state = self.resources.get_mut::<TurnState>().unwrap();
                //*change_state = TurnState::PlayerTurn;
                self.round_start_systems.execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::AwaitingInput => {self.input_systems.execute(&mut self.ecs, &mut self.resources);},
            TurnState::PlayerTurn => {self.player_systems.execute(&mut self.ecs, &mut self.resources);},
            TurnState::EnemyTurn => {self.enemy_systems.execute(&mut self.ecs, &mut self.resources);},
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
