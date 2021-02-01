use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect;

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator, options: &GameOptions) -> MapBuilder {
        println!("Running EmptyArchitect");
        let mut mb = MapBuilder::new();

        mb.map.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.map.find_most_distant_from(mb.player_start);
        mb.monster_spawns = mb.spawn_locations(mb.player_start.clone(), options, rng, 5);

        mb
    }
}
