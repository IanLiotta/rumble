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
        .for_each(|(entity, enemy)| commands.remove(*entity));
    <(Entity, &Spawner)>::query()
        .iter(ecs)
        .for_each(|(entity, enemy)| commands.remove(*entity));
    <(Entity, &Player)>::query()
        .iter(ecs)
        .for_each(|(entity, player)| commands.remove_component::<Point>(*entity));
    //Clear out the turn queue
    turn_queue.queue = VecDeque::new();
    // regenerate the map out in the main loop
}
