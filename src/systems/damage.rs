use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(DirectDamage)]
#[write_component(Health)]
#[read_component(Score)]
#[read_component(Player)]
pub fn damage(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_queue: &mut TurnQueue,
) {
    let _attacks = <(Entity, &Health, &DirectDamage, Option<&Enemy>)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, health, directdamage, enemy)| {
            let new_hp = health.hp - directdamage.amount;
            if new_hp <= 0 {
                // because of the borrow checker we can't get the player here
                // so send a signal to add score in another system.
                if let Some(enemy) = enemy {
                    commands.push(((), AddScore { score: enemy.value }));
                }
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
                commands.add_component(
                    *entity,
                    Health {
                        hp: new_hp,
                        max_hp: health.max_hp,
                    },
                );
                commands.remove_component::<DirectDamage>(*entity);
            }
        });
}
