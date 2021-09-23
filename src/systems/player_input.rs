// Accept mouse input to move player
// Also discover the movement range of the player
// and add it as a component to the player entity

use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource]map: &Map,
    #[resource]turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) 
{
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mouse_pos = INPUT.lock().mouse_tile(0);
    let mouse_idx = Map::map_idx(mouse_pos.x as usize, mouse_pos.y as usize);
    // find each player controlled entity
    players.iter(ecs).for_each(|(player, player_pos)| 
    {
        // find the valid moves within the player's movement range
        let possible_moves = tiles_in_range(map, 2.0, Map::map_idx(player_pos.x as usize, player_pos.y as usize));
        // queue command to add the MovementRange component to the player entity
        commands.add_component(*player, MovementRange{move_range: possible_moves.clone()});
        // if a valid tile is clicked, queue a message-entity that the player wants to move
        if INPUT.lock().is_mouse_button_pressed(0) && possible_moves.contains(&mouse_idx)
        {
            commands.push(((), WantsToMove{entity: *player, destination: mouse_pos}));
            *turn_state = TurnState::PlayerTurn;
        }
    });
}