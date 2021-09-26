use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn render_hud() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.draw_hollow_box(Rect::with_size(0, 50, 79, 9), ColorPair::new(BLUE, BLACK));
    draw_batch.draw_hollow_box(Rect::with_size(65, 0, 14, 49), ColorPair::new(RED, BLACK));
    draw_batch.print_color_centered(50, "Test HUD", ColorPair::new(RED, WHITE));
    draw_batch.submit(2200).expect("Batch error");
}
