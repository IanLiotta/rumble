use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub obstacles: Vec<Rect>,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut mb = MapBuilder {
            map: Map::new(),
            obstacles: Vec::new(),
        };
        mb.enclose_map();
        mb.random_obstacles(&mut rng);
        mb
    }

    fn enclose_map(&mut self) {
        for i in 0..ARENA_WIDTH {
            let fill_idx1 = Map::map_idx(i, 0);
            let fill_idx2 = Map::map_idx(i, ARENA_HEIGHT - 1);
            self.map.tiles[fill_idx1] = TileType::Wall;
            self.map.tiles[fill_idx2] = TileType::Wall;
        }
        for i in 0..ARENA_HEIGHT {
            let fill_idx1 = Map::map_idx(0, i);
            let fill_idx2 = Map::map_idx(ARENA_HEIGHT - 1, i);
            self.map.tiles[fill_idx1] = TileType::Wall;
            self.map.tiles[fill_idx2] = TileType::Wall;
        }
    }

    fn random_obstacles(&mut self, rng: &mut RandomNumberGenerator) {
        for _i in 0..rng.range(4, 12) {
            let obs = Rect::with_size(
                rng.range(1, ARENA_WIDTH - 5),
                rng.range(1, ARENA_HEIGHT - 5),
                rng.range(2, 8),
                rng.range(2, 8),
            );
            self.obstacles.push(obs);
            obs.for_each(|o| {
                if o.x > 0 && o.x < ARENA_WIDTH as i32 && o.y > 0 && o.y < ARENA_HEIGHT as i32 {
                    let idx = Map::map_idx(o.x as usize, o.y as usize);
                    self.map.tiles[idx] = TileType::Wall;
                }
            });
        }
    }
}
