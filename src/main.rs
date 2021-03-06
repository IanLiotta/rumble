mod components;
mod map;
mod map_builder;
mod movement_range;
mod systems;
mod turn_queue;
mod turn_state;

mod prelude {
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::movement_range::*;
    pub use crate::systems::*;
    pub use crate::turn_queue::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use std::collections::VecDeque;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 40;
    pub const ARENA_HEIGHT: usize = 40;
    pub const ARENA_WIDTH: usize = 40;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    elapsed_frame_time: f32,
    round_start_systems: Schedule,
    input_systems: Schedule,
    targeting_systems: Schedule,
    player_systems: Schedule,
    enemy_systems: Schedule,
    game_over_systems: Schedule,
    shop_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let world = World::default();
        let mut resources = Resources::default();
        let mb = MapBuilder::new();
        resources.insert(mb.map);
        resources.insert(TurnState::StartGame);
        resources.insert(TurnQueue {
            queue: VecDeque::new(),
        });
        State {
            ecs: world,
            resources: resources,
            elapsed_frame_time: 0.0,
            round_start_systems: build_round_start_scheduler(),
            input_systems: build_input_scheduler(),
            targeting_systems: build_targeting_scheduler(),
            player_systems: build_player_scheduler(),
            enemy_systems: build_enemy_scheduler(),
            game_over_systems: build_game_over_scheduler(),
            shop_systems: build_shop_scheduler(),
        }
    }
}

impl GameState for State {
    // run by main_loop
    fn tick(&mut self, ctx: &mut BTerm) {
        self.elapsed_frame_time += ctx.frame_time_ms;
        // watch the input event queue for quit events, pass the rest along to the input system
        let mut input_events = std::collections::VecDeque::<BEvent>::new();
        while let Some(event) = INPUT.lock().pop() {
            match event {
                BEvent::CloseRequested => {
                    ctx.quitting = true;
                }
                _ => {
                    input_events.push_back(event);
                }
            }
        }
        self.resources.insert(input_events);
        //if self.elapsed_frame_time > FRAME_TIME {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::StartGame => {
                self.round_start_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::AwaitingInput => {
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::PlayerTargeting => {
                self.targeting_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::EnemyTurn => {
                self.enemy_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::GameOver => {
                self.game_over_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::Shop => {
                let mb = MapBuilder::new();
                self.resources.insert(mb.map);
                self.shop_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
        }
        //if self.elapsed_frame_time > FRAME_TIME {
        //render_entities(&mut self.ecs);
        //self.elapsed_frame_time = 0.0;
        //}
        render_attacks(&mut self.ecs);
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
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_sparse_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_advanced_input(true)
        .build()?;
    let gs = State::new();
    main_loop(context, gs)
}

pub fn render_attacks(ecs: &mut World) {
    let mut commands = CommandBuffer::new(ecs);
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    <(Entity, &DrawLine)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, line)| {
            Bresenham::new(line.source, line.dest).for_each(|point| {
                draw_batch.set(point, ColorPair::new(RED, BLACK), to_cp437('+'));
            });
            if line.duration == 0 {
                commands.remove(*entity);
            } else {
                commands.add_component(
                    *entity,
                    DrawLine {
                        source: line.source,
                        dest: line.dest,
                        duration: line.duration - 1,
                    },
                );
            }
            draw_batch.submit(3000).expect("Batch error");
        });
    commands.flush(ecs);
}
