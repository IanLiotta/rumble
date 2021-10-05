use crate::prelude::*;

// Rotates through turns that should only last one iteration of their schedule,
// or idles on ones that can persist until something like input happens.
#[system]
#[read_component(Player)]
#[read_component(IsMoving)]
#[read_component(WantsToAttack)]
#[read_component(WantsToMove)]
#[read_component(WantsToLeave)]
#[read_component(WantsToPlay)]
#[read_component(Enemy)]
#[read_component(Spawner)]
#[write_component(FieldOfView)]
pub fn end_turn(
    ecs: &mut SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] turn_queue: &mut TurnQueue,
    commands: &mut CommandBuffer,
) {
    let new_state = match turn_state {
        TurnState::StartGame => {
            let entry = ecs.entry_ref(turn_queue.queue[0]).unwrap();
            if let Ok(_entry) = entry.get_component::<Player>() {
                TurnState::AwaitingInput
            } else {
                TurnState::EnemyTurn
            }
        }
        TurnState::AwaitingInput => {
            // See if the player wants to leave
            let leaving = <&WantsToLeave>::query().iter(ecs).nth(0);
            if let Some(_leaving) = leaving {
                // Get rid of enemies and spawners
                <(Entity, &Enemy)>::query()
                    .iter(ecs)
                    .for_each(|(entity, _enemy)| commands.remove(*entity));
                <(Entity, &Spawner)>::query()
                    .iter(ecs)
                    .for_each(|(entity, _spawner)| commands.remove(*entity));
                // clear the turn queue, then put the player back into it
                // mark the player's fov as dirty
                turn_queue.queue = VecDeque::new();
                let player = <(Entity, &Player, &mut FieldOfView)>::query()
                    .iter_mut(ecs)
                    .nth(0);
                if let Some(player) = player {
                    player.2.is_dirty = true;
                    turn_queue.queue.push_back(*player.0);
                }
                TurnState::Shop
            } else {
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
        }
        TurnState::PlayerTargeting => {
            let attack_request = <&WantsToAttack>::query().iter(ecs).nth(0);
            if let Some(_attack_request) = attack_request {
                TurnState::PlayerTargeting
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
        TurnState::PlayerTurn => {
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
        TurnState::EnemyTurn => {
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
        TurnState::GameOver => return,
        TurnState::Shop => {
            let wants_to_play = <(Entity, &WantsToPlay)>::query().iter(ecs).nth(0);
            if let Some(wants_to_play) = wants_to_play {
                TurnState::StartGame
            } else {
                return;
            }
        }
    };
    *turn_state = new_state;
}
