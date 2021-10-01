use crate::prelude::*;

#[system]
pub fn game_over() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_color_centered(30, "Game Over", ColorPair::new(WHITE, RED));
    draw_batch.submit(2200).expect("Batch error");
}
