use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Enemy)]
#[read_component(DirectDamage)]
pub fn damage(
    ecs: &mut SubWorld,
    #[resource]map: &Map,
    commands: &mut CommandBuffer,
) {
    let mut attacks = <(Entity, &DirectDamage)>::query().iter(ecs).for_each(|(entity, directdamage)| {
        commands.remove(*entity);
    });

    
}