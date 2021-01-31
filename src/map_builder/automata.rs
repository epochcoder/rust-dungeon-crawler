use crate::prelude::*;

use super::MapArchitect;

pub struct CellularAutomataArchitect;

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|tile| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *tile = TileType::Floor
            } else {
                *tile = TileType::Wall
            }
        });
    }

    fn iteration(&self, map: &mut Map) {
        // we clone so we don't count neighbors on a mpa we are currently changing
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let n = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                if n > 4 || n == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }

        map.tiles = new_tiles;
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for py in -1..=1 {
            for px in -1..=1 {
                if x != 0 && y != 0 && map.tiles[map_idx(x + px, y + py)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn find_closest_point_to(&self, point: Point, map: &Map) -> Option<Point> {
        map.distance_from_point(point)
            .iter()
            .min_by(|(_, distance1), (_, distance2)| {
                f32::partial_cmp(distance1, distance2).unwrap()
            })
            .map(|(idx, _)| map.index_to_point2d(*idx))
    }

    fn find_furthest_point_from(&self, point: Point, map: &Map) -> Option<Point> {
        map.distance_from_point(point)
            .iter()
            .max_by(|(_, distance1), (_, distance2)| {
                f32::partial_cmp(distance1, distance2).unwrap()
            })
            .map(|(idx, _)| map.index_to_point2d(*idx))
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator, options: &GameOptions) -> MapBuilder {
        let mut mb = MapBuilder::new();

        self.random_noise_map(rng, &mut mb.map);
        self.iteration(&mut mb.map);

        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.player_start = self.find_closest_point_to(center, &mb.map).unwrap();
        mb.amulet_start = self.find_furthest_point_from(mb.player_start, &mb.map).unwrap();
        mb.monster_spawns = mb.spawn_locations(mb.player_start.clone(), options, rng);

        mb
    }
}
