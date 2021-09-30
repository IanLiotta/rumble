use crate::prelude::*;

// Rotates through turns that should only last one iteration of their schedule,
// or idles on ones that can persist until something like input happens.
#[system]
#[read_component(Player)]
#[read_component(IsMoving)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::StartGame => TurnState::AwaitingInput,
        TurnState::AwaitingInput => return,
        TurnState::PlayerTargeting => TurnState::EnemyTurn,
        TurnState::PlayerTurn => {
            let movers = <&IsMoving>::query().iter(ecs).nth(0);
            if let Some(_mover) = movers {
                TurnState::PlayerTurn
            } else {
                TurnState::EnemyTurn
            }
        }
        TurnState::EnemyTurn => {
            let movers = <&IsMoving>::query().iter(ecs).nth(0);
            if let Some(_mover) = movers {
                TurnState::EnemyTurn
            } else {
                TurnState::AwaitingInput
            }
        }
    };

    *turn_state = new_state;
}
