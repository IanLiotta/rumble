use crate::prelude::*;

#[system]
pub fn round_start(commands: &mut CommandBuffer, #[resource] map: &Map) {
    create_spawners(commands, map);
}

fn create_spawners(commands: &mut CommandBuffer, map: &Map) {
    let spawner_origins = [
        (1, 1),
        (1, ARENA_HEIGHT - 1),
        (ARENA_WIDTH - 1, 1),
        (ARENA_HEIGHT - 1, ARENA_WIDTH - 1),
    ];
    for mut loc in spawner_origins {
        let mut rect = Rect::with_size(loc.0, loc.1, 2, 2);
        let mut placed = false;
        while !placed {
            rect.for_each(|point| {
                let idx = Map::map_idx(point.x as usize, point.y as usize);
                if !placed
                    && point.x > 0
                    && point.x < ARENA_WIDTH as i32
                    && point.y > 0
                    && point.y < ARENA_HEIGHT as i32
                    && map.tiles[idx] == TileType::Floor
                {
                    commands.push((
                        Spawner {},
                        Point::new(point.x, point.y),
                        Render {
                            color: ColorPair::new(GREEN, BLACK),
                            glyph: to_cp437('^'),
                        },
                    ));
                    placed = true;
                }
            });
            if !placed {
                if loc.0 > ARENA_WIDTH / 2 {
                    loc.0 -= 1
                } else {
                    loc.0 += 1
                }
                if loc.1 > ARENA_HEIGHT / 2 {
                    loc.1 -= 1
                } else {
                    loc.1 += 1
                }
                rect = Rect::with_size(
                    loc.0,
                    loc.1,
                    rect.width() as usize + 1,
                    rect.height() as usize + 1,
                )
            }
        }
    }
}
