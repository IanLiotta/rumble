use crate::prelude::*;

// make the enemies walk randomly

#[system(for_each)]
#[read_component(Enemy)]
#[read_component(Point)]
#[read_component(MovesRandomly)]
pub fn random_walk(#[resource]map: &Map, entity: &Entity, start_loc: &Point, commands: &mut CommandBuffer)
{
    let mut rng = RandomNumberGenerator::new();
    //Pick a random direction to move
    let destination = match rng.range(0,4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        _ => Point::new(0, 1),
    } + *start_loc;
    commands.push(((), WantsToMove{entity: *entity, destination}));
}