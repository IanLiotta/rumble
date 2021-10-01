use crate::prelude::*;

// Rotates through turns that should only last one iteration of their schedule,
// or idles on ones that can persist until something like input happens.
#[system]
#[read_component(Player)]
#[read_component(IsMoving)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] turn_queue: &mut TurnQueue,
) {
    let new_state = match turn_state {
        TurnState::StartGame => TurnState::AwaitingInput,
        TurnState::AwaitingInput => return,
        TurnState::PlayerTargeting => TurnState::EnemyTurn,
        TurnState::PlayerTurn => {
            // if someone is moving, keep cycling this state until they're done
            let movers = <&IsMoving>::query().iter(ecs).nth(0);
            if let Some(_mover) = movers {
                TurnState::PlayerTurn
            } else {
                // if movement is done, check to see if the next entity is a player or enemy and go to their turn
                let just_acted = turn_queue.current.unwrap();
                turn_queue.queue.push_back(just_acted);
                turn_queue.current = turn_queue.queue.pop_front();
                let next = turn_queue.current.unwrap();
                let entry = ecs.entry_ref(next).unwrap();
                if let Ok(_entry) = entry.get_component::<Player>() {
                    TurnState::AwaitingInput
                } else {
                    TurnState::EnemyTurn
                }
            }
        }
        TurnState::EnemyTurn => {
            // if someone is moving, keep cycling this state until they're done
            let movers = <&IsMoving>::query().iter(ecs).nth(0);
            if let Some(_mover) = movers {
                TurnState::EnemyTurn
            } else {
                // if movement is done, check to see if the next entity is a player or enemy and go to their turn
                let just_acted = turn_queue.current.unwrap();
                turn_queue.queue.push_back(just_acted);
                turn_queue.current = turn_queue.queue.pop_front();
                let next = turn_queue.current.unwrap();
                let entry = ecs.entry_ref(next).unwrap();
                if let Ok(_entry) = entry.get_component::<Player>() {
                    TurnState::AwaitingInput
                } else {
                    TurnState::EnemyTurn
                }
            }
        }
    };
    *turn_state = new_state;
}
