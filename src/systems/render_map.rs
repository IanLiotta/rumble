use crate::prelude::*;

#[system]
pub fn render_map(#[resource] map: &Map) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for idx in 0..map.tiles.len() {
        let tile_point = Map::map_idx2point(idx);
        let glyph = match map.tiles[idx] {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
        };
        draw_batch.set(
            tile_point,
            ColorPair::new(WHITE, BLACK),
            glyph
        );
    }
    draw_batch.submit(0).expect("Batch error");
}