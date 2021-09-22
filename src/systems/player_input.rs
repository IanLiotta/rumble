use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mouse_pos = INPUT.lock().mouse_tile(0);
    let mouse_idx = Map::map_idx(mouse_pos.x as usize, mouse_pos.y as usize);
    if INPUT.lock().is_mouse_button_pressed(0){
        //commands.push(((), WantsToMove{entity: player, destination: mouse_pos}));
    }
}