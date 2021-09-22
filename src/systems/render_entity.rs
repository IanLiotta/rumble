use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn render_entity(ecs: &SubWorld, #[resource]map: &Map){
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)|{
            draw_batch.set(
                *pos,
                render.color,
                render.glyph
            );
        });
    draw_batch.submit(1700).expect("Batch error");
}