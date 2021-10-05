use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(Spawner)]
#[read_component(Player)]
#[read_component(WantsToLeave)]
pub fn round_end(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_queue: &mut TurnQueue,
) {
    //Delete the enemies and spawners
    <(Entity, &Enemy)>::query()
        .iter(ecs)
        .for_each(|(entity, _enemy)| commands.remove(*entity));
    <(Entity, &Spawner)>::query()
        .iter(ecs)
        .for_each(|(entity, _spawner)| commands.remove(*entity));
    //Clear out the turn queue
    turn_queue.queue = VecDeque::new();
    let player = <(Entity, &Player)>::query().iter(ecs).nth(0);
    if let Some(player) = player {
        turn_queue.queue.push_back(*player.0);
    }
    // regenerate the map out in the main loop
}
