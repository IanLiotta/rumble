use crate::prelude::*;

#[system]
#[write_component(Health)]
#[read_component(Player)]
pub fn shop(
    ecs: &SubWorld,
    #[resource] input_events: &mut std::collections::VecDeque<BEvent>,
    commands: &mut CommandBuffer,
    #[resource] turn_queue: &mut TurnQueue,
) {
    // Draw the Shop Menu
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Shoppe Goes Here");
    draw_batch.print_centered(3, "1. Repair");
    draw_batch.print_centered(8, "6. Return");
    draw_batch.submit(2200).expect("Batch error");

    //grab the player entity so we can fix it up
    let player = <(Entity, &Player, &Health)>::query()
        .iter(ecs)
        .nth(0)
        .unwrap();
    let (player_entity, _, player_health) = player;
    while let Some(event) = input_events.pop_front() {
        match event {
            // Repair player to full hp
            BEvent::KeyboardInput {
                key: VirtualKeyCode::Key1,
                pressed: true,
                ..
            } => {
                commands.add_component(
                    *player_entity,
                    Health {
                        hp: player_health.max_hp,
                        max_hp: player_health.max_hp,
                    },
                );
            }
            // Start the next round of gameplay
            BEvent::KeyboardInput {
                key: VirtualKeyCode::Key6,
                pressed: true,
                ..
            } => {
                commands.push(((), WantsToPlay {}));
            }
            _ => {}
        }
    }
}
