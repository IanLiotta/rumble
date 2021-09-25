use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToAttack)]
#[read_component(Point)]
pub fn targeting(
    ecs: &mut SubWorld,
    entity: &Entity,
    attacker: &WantsToAttack,
    commands: &mut CommandBuffer,
    #[resource]map: &Map,
    #[resource]input_events: &mut std::collections::VecDeque<BEvent>,
    #[resource]turn_state: &mut TurnState,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let target_tiles = tiles_in_range(map, 10.0, Map::map_idx(attacker.pos.x as usize, attacker.pos.y as usize));
    // exclude the 0th target_tile, that's the player
    target_tiles[1..].iter().for_each(|target|{
        draw_batch.set(
            Map::map_idx2point(*target),
            ColorPair::new(RGBA::from_f32(1.0, 0.0, 0.0, 0.5), BLACK),
            to_cp437('.')
        );
    });

    let input = INPUT.lock();
    let mouse_pos = input.mouse_tile(0);
    let mouse_idx = Map::map_idx(mouse_pos.x as usize, mouse_pos.y as usize);
    while let Some(event) = input_events.pop_front() {
        match event {
            BEvent::MouseButtonDown{button: 0} => {
                if target_tiles.contains(&mouse_idx) {
                    println!("Valid Target");
                    commands.remove(*entity);
                    *turn_state = TurnState::EnemyTurn;
                } else {
                    *turn_state = TurnState::AwaitingInput;
                }
            },
            _ => {}
        }
    }

    draw_batch.submit(1900).expect("Batch error");
}