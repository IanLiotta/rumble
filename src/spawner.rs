use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            MovementRange {move_range: Vec::new()},
            Render{
                color: ColorPair::new(RED, BLACK),
                glyph: to_cp437('@'),
            }
        )
    );
}