use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(DirectDamage)]
#[write_component(Health)]
pub fn damage(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let _attacks = <(Entity, &Health, &DirectDamage)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, health, directdamage)| {
            let new_hp = health.hp - directdamage.amount;
            if new_hp <= 0 {
                commands.remove(*entity);
            } else {
                commands.add_component(*entity, Health { hp: new_hp });
                commands.remove_component::<DirectDamage>(*entity);
            }
        });
}
