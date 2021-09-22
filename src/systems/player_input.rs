use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource]map: &Map,
    commands: &mut CommandBuffer,
) 
{
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mouse_pos = INPUT.lock().mouse_tile(0);
    let mouse_idx = Map::map_idx(mouse_pos.x as usize, mouse_pos.y as usize);
    players.iter(ecs).for_each(|(player, player_pos)| 
    {
        let possible_moves = tiles_in_range(map, 2.0, Map::map_idx(player_pos.x as usize, player_pos.y as usize));
        commands.add_component(*player, MovementRange{move_range: possible_moves.clone()});
        if INPUT.lock().is_mouse_button_pressed(0) && possible_moves.contains(&mouse_idx)
        {
            commands.push(((), WantsToMove{entity: *player, destination: mouse_pos}));
        }
    });
}