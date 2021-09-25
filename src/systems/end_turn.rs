use crate::prelude::*;

// Rotates through turns that should only last one iteration of their schedule,
// or idles on ones that can persist until something like input happens.
#[system]
#[read_component(Player)]
pub fn end_turn(#[resource]turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::StartGame => TurnState::AwaitingInput,
        TurnState::AwaitingInput => return,
        TurnState::PlayerTargeting => TurnState::EnemyTurn,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
    };

    *turn_state = new_state;
}