use crate::prelude::*;

pub fn draw_sprite(spritesheet: &Texture2D, spr_idx: i32, x: f32, y: f32){
    let spr_x: f32 = (spr_idx % SPRITESHEET_WIDTH) as f32 * 32.;
    let spr_y: f32 = (spr_idx / SPRITESHEET_HEIGHT) as f32 * 32.;
    draw_texture_ex(
        *spritesheet,
        x * TILE_SIZE,
        y * TILE_SIZE,
        macroquad::color::WHITE,
        DrawTextureParams{
            source: Some(macroquad::math::Rect::new(spr_x, spr_y, 32., 32.)),
            ..Default::default()
        }
    );
}