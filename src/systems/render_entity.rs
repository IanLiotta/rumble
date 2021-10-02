use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(DrawOffset)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn render_entity(ecs: &SubWorld) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos, render.color, render.glyph);
        });
    draw_batch.submit(2100).expect("Batch error");
}
