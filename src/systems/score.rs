use crate::prelude::*;

#[system]
#[write_component(Score)]
#[read_component(Player)]
#[read_component(AddScore)]
pub fn score(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // collect all outstanding score changes into a vec
    let add_score: Vec<(Entity, i32)> = <(Entity, &AddScore)>::query()
        .iter(ecs)
        .map(|(ent, add_score)| (*ent, add_score.score))
        .collect();
    // get the player's score component
    let score = <&mut Score>::query()
        .filter(component::<Player>())
        .iter_mut(ecs)
        .next()
        .unwrap();
    add_score.iter().for_each(|(ent, value)| {
        score.current += value;
        score.max += value;
        commands.remove(*ent);
    });
}
