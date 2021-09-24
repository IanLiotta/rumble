use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn render_hud() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_color_centered(0, "Test HUD", ColorPair::new(RED,WHITE));
    draw_batch.submit(1700).expect("Batch error");
}