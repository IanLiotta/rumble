use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[write_component(DrawOffset)]
pub fn render_entity(ecs: &mut SubWorld){
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    <(&Point, &Render, &mut DrawOffset)>::query()
        .iter_mut(ecs)
        .for_each(|(pos, render, offset)|{
            let offset_pos_x = pos.x as f32 + offset.offset_x;
            let offset_pos_y = pos.y as f32 + offset.offset_y + 1.; // being drawn off by one with an offset of 0.0, why?
            let offset_pos = PointF::new(offset_pos_x, offset_pos_y);
            draw_batch.set_fancy(
                offset_pos,
                0,
                Radians::new(0.0),
                PointF::new(1.,1.),
                render.color,
                render.glyph
            ); 
            if offset.offset_x != 0.0 {offset.offset_x -= offset.offset_x.signum()*0.1};
            if offset.offset_y != 0.0 {offset.offset_y -= offset.offset_y.signum()*0.1};

        });
    draw_batch.submit(1700).expect("Batch error");
}