use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(DirectDamage)]
#[write_component(Health)]
pub fn damage(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_queue: &mut TurnQueue,
) {
    let _attacks = <(Entity, &Health, &DirectDamage)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, health, directdamage)| {
            let new_hp = health.hp - directdamage.amount;
            if new_hp <= 0 {
                if turn_queue.queue.len() > 0 {
                    for i in 0..turn_queue.queue.len() {
                        if turn_queue.queue[i] == *entity {
                            turn_queue.queue.remove(i);
                            break;
                        }
                    }
                }
                commands.remove(*entity);
            } else {
                commands.add_component(*entity, Health { hp: new_hp });
                commands.remove_component::<DirectDamage>(*entity);
            }
        });
}
