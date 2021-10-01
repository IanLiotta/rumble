use crate::prelude::*;

// Rotates through turns that should only last one iteration of their schedule,
// or idles on ones that can persist until something like input happens.
#[system]
#[read_component(Player)]
#[read_component(IsMoving)]
#[read_component(WantsToAttack)]
#[read_component(WantsToMove)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] turn_queue: &mut TurnQueue,
) {
    let new_state = match turn_state {
        TurnState::StartGame => TurnState::AwaitingInput,
        TurnState::AwaitingInput => {
            let move_request = <&WantsToMove>::query().iter(ecs).nth(0);
            if let Some(_move_request) = move_request {
                TurnState::PlayerTurn
            } else {
                let attack_request = <&WantsToAttack>::query().iter(ecs).nth(0);
                if let Some(_attack_request) = attack_request {
                    TurnState::PlayerTargeting
                } else {
                    TurnState::AwaitingInput
                }
            }
        }
        TurnState::PlayerTargeting => {
            let attack_request = <&WantsToAttack>::query().iter(ecs).nth(0);
            if let Some(_attack_request) = attack_request {
                TurnState::PlayerTargeting
            } else {
                TurnState::EnemyTurn
            }
        }
        TurnState::PlayerTurn => {
            if turn_queue.queue.len() <= 1 {
                TurnState::GameOver
            } else {
                // if someone is moving, keep cycling this state until they're done
                let movers = <&IsMoving>::query().iter(ecs).nth(0);
                if let Some(_mover) = movers {
                    TurnState::PlayerTurn
                } else {
                    // if movement is done, check to see if the next entity is a player or enemy and go to their turn
                    turn_queue.queue.push_back(turn_queue.queue[0]);
                    turn_queue.queue.pop_front();
                    let entry = ecs.entry_ref(turn_queue.queue[0]).unwrap();
                    if let Ok(_entry) = entry.get_component::<Player>() {
                        TurnState::AwaitingInput
                    } else {
                        TurnState::EnemyTurn
                    }
                }
            }
        }
        TurnState::EnemyTurn => {
            if turn_queue.queue.len() <= 1 {
                TurnState::GameOver
            } else {
                // if someone is moving, keep cycling this state until they're done
                let movers = <&IsMoving>::query().iter(ecs).nth(0);
                if let Some(_mover) = movers {
                    TurnState::EnemyTurn
                } else {
                    // if movement is done, check to see if the next entity is a player or enemy and go to their turn
                    turn_queue.queue.push_back(turn_queue.queue[0]);
                    turn_queue.queue.pop_front();
                    let entry = ecs.entry_ref(turn_queue.queue[0]).unwrap();
                    if let Ok(_entry) = entry.get_component::<Player>() {
                        TurnState::AwaitingInput
                    } else {
                        TurnState::EnemyTurn
                    }
                }
            }
        }
        TurnState::GameOver => return,
    };
    *turn_state = new_state;
}
