use crate::prelude::*;

use super::MapArchitect;

pub struct DrunkardArchitect {}

const STAGGER_DISTANCE: usize = 400;
const OPEN_PERCENTAGE: f32 = 33.33;

impl DrunkardArchitect {
    fn drunkard(
        &self,
        start: &Point,
        map: &mut Map,
        rng: &mut RandomNumberGenerator,
    ) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            map.set_tile(drunkard_pos, TileType::Floor);

            let next_point_delta = match rng.range(0, 4) {
                0 => Point::new(0, 1),
                1 => Point::new(0, -1),
                2 => Point::new(1, 0),
                _ => Point::new(-1, 0),
            };

            let dest = drunkard_pos + next_point_delta;
            if !map.in_bounds(dest) {
                break;
            }

            distance_staggered += 1;
            drunkard_pos = dest;

            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }

    fn carve_map(&self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, map, rng); //just so player can actually walk

        loop {
            // keep starting from a random point on the map
            self.drunkard(&Point::new(
                rng.range(0, SCREEN_WIDTH),
                rng.range(0, SCREEN_HEIGHT),
            ), map, rng);

            let dijsktra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![map.point2d_to_index(center)],
                map,
                1024.0,
            );

            // close tiles which are inaccessible (since our drunkard started digging from random places)
            dijsktra_map.map.iter()
                .enumerate()
                .filter(|(_, dist)| *dist > &2000.0)
                .for_each(|(idx, _)| map.set_tile(map.index_to_point2d(idx), TileType::Wall));

            let open_tiles = map.tiles
                .iter()
                .filter(|t| **t == TileType::Floor)
                .count() as f32;

            let completed_percentage = open_tiles / (SCREEN_WIDTH * SCREEN_HEIGHT) as f32 * 100f32;
            println!("Drunkard finished carving, {:?}%", completed_percentage);

            if completed_percentage >= OPEN_PERCENTAGE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator, options: &GameOptions) -> MapBuilder {
        println!("Running DrunkardArchitect");
        let mut mb = MapBuilder::new();

        mb.map.fill(TileType::Wall);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        self.carve_map(&mut mb.map, rng);

        mb.monster_spawns = mb.spawn_locations(mb.player_start.clone(), options, rng, DEFAULT_NUM_MONSTERS);
        mb.amulet_start = mb.map.find_most_distant_from(mb.player_start.clone());

        mb
    }
}
