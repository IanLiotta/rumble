use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(DrawOffset)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn render_entity(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut renderables = <(Entity, &Point, &Render, &DrawOffset)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    renderables
        .iter(ecs)
        .filter(|(_, pos, _, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(entity, pos, render, offset)| {
            let offset_pos_x = pos.x as f32 + offset.offset_x;
            let offset_pos_y = pos.y as f32 + offset.offset_y + 1.; // being drawn off by one with an offset of 0.0, why?
            let offset_pos = PointF::new(offset_pos_x, offset_pos_y);
            draw_batch.set_fancy(
                offset_pos,
                0,
                Radians::new(0.0),
                PointF::new(1., 1.),
                render.color,
                render.glyph,
            );
            let new_offset_x = if offset.offset_x.abs() >= 0.1 {
                offset.offset_x - offset.offset_x.signum() * 0.1
            } else {
                0.0
            };
            let new_offset_y = if offset.offset_y.abs() >= 0.1 {
                offset.offset_y - offset.offset_y.signum() * 0.1
            } else {
                0.0
            };
            commands.add_component(
                *entity,
                DrawOffset {
                    offset_x: new_offset_x,
                    offset_y: new_offset_y,
                },
            )
        });
    draw_batch.submit(2100).expect("Batch error");
}
