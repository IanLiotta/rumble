use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mouse_pos = INPUT.lock().mouse_tile(0);
    if INPUT.lock().is_mouse_button_pressed(0){
        players.iter(ecs).for_each(|(player, _)| {
            commands.push(((), WantsToMove{entity: *player, destination: mouse_pos}));
        });
    }
}