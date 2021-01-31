mod automata;
mod empty;
mod rooms;

use crate::prelude::*;

use automata::CellularAutomataArchitect;
use empty::EmptyArchitect;
use rooms::RoomsArchitect;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator, options: &GameOptions) -> MapBuilder;
}

pub enum TunnelType {
    Horizontal,
    Vertical,
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    fn new() -> Self {
        Self {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        }
    }

    pub fn build(rng: &mut RandomNumberGenerator, options: &GameOptions) -> Self {
        let mut architect = CellularAutomataArchitect {};
        let builder = architect.new(rng, options);

        builder
    }

    /// collect locations of possible spawn locations
    /// on the map for monsters which are not
    /// too close to the player
    pub fn spawn_locations(&self, start: Point, options: &GameOptions, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let mut spawn_locations: Vec<Point> = self.map.points_further_than(start, (options.player_fov + 2) as f32);
        let mut spawns: Vec<Point> = Vec::new();

        for _ in 0..DEFAULT_NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawn_locations).unwrap();
            spawns.push(spawn_locations[target_index].clone());
            spawn_locations.remove(target_index);
        }

        spawns
    }
}
