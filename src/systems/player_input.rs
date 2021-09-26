// Accept mouse input to move player
// Also discover the movement range of the player
// and add it as a component to the player entity

use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(FieldOfView)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource]map: &Map,
    #[resource]turn_state: &mut TurnState,
    #[resource]input_events: &mut std::collections::VecDeque<BEvent>,
    commands: &mut CommandBuffer,
) 
{
    let input = INPUT.lock();
    let mut mobs = <(Entity, &Point)>::query();
    let mut mobs_idx = Vec::new();
    mobs.iter(ecs).for_each(|(_, mob_point)| {
        mobs_idx.push(Map::map_idx(mob_point.x as usize, mob_point.y as usize));
    });
    let mut players = mobs.filter(component::<Player>());
    let mouse_pos = input.mouse_tile(0);
    let mouse_idx = Map::map_idx(mouse_pos.x as usize, mouse_pos.y as usize);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    // find each player controlled entity
    players.iter(ecs).for_each(|(player, player_pos)| 
    {
        // find the valid moves within the player's movement range
        let mut possible_moves: Vec<usize> = vec![];
        tiles_in_range(map, 10.0, Map::map_idx(player_pos.x as usize, player_pos.y as usize)).iter().for_each(|tile| {
            if player_fov.visible_tiles.contains(&Map::map_idx2point(*tile)) {
                possible_moves.push(*tile);
            }
        });
        // queue command to add the MovementRange component to the player entity
        commands.add_component(*player, MovementRange{move_range: possible_moves.clone()});
        // if a valid tile is clicked, queue a message-entity that the player wants to move
        while let Some(event) = input_events.pop_front() {
            match event {
                BEvent::MouseButtonDown{button: 0} => {
                    if !mobs_idx.contains(&mouse_idx) && possible_moves.contains(&mouse_idx) {
                        commands.push(((), WantsToMove{entity: *player, source:*player_pos, destination: mouse_pos}));
                        *turn_state = TurnState::PlayerTurn;
                    }      
                },
                BEvent::KeyboardInput{key: VirtualKeyCode::Key1, pressed:true, ..} => {
                    commands.push(((), WantsToAttack{attacker: *player, pos: *player_pos}));
                    *turn_state = TurnState::PlayerTargeting;
                },
                _ => {}
            }
        }
    });
}