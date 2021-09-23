use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn render_entity(ecs: &SubWorld, #[resource]spritesheet: &Texture2D){
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)|{
            draw_sprite(spritesheet, render.spr_idx, pos.x as f32, pos.y as f32)
        });
}