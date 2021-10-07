use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(DirectDamage)]
#[read_component(DrainEnergy)]
#[write_component(Health)]
#[write_component(Energy)]
#[read_component(Score)]
#[read_component(Player)]
pub fn damage(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_queue: &mut TurnQueue,
) {
    let _attacks = <(
        Entity,
        &Health,
        &Energy,
        Option<&DirectDamage>,
        Option<&DrainEnergy>,
        Option<&Enemy>,
    )>::query()
    .iter_mut(ecs)
    .for_each(
        |(entity, health, energy, direct_damage, drain_energy, enemy)| {
            if let Some(direct_damage) = direct_damage {
                let new_hp = health.hp - direct_damage.amount;
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
            } // end Direct Damage handling
            if let Some(drain_energy) = drain_energy {
                let new_energy = energy.energy - drain_energy.amount;
                if new_energy > 0 {
                    commands.add_component(
                        *entity,
                        Energy {
                            energy: new_energy,
                            max_energy: energy.max_energy,
                        },
                    )
                } else {
                    commands.add_component(
                        *entity,
                        Energy {
                            energy: 0,
                            max_energy: energy.max_energy,
                        },
                    )
                }
                commands.remove_component::<DrainEnergy>(*entity);
            } // end drain energy handling
        },
    ); // end for_each
}
