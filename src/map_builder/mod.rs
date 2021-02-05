mod automata;
mod empty;
mod rooms;
mod drunkard;
mod prefab;
mod themes;

use crate::prelude::*;

use prefab::*;
use automata::CellularAutomataArchitect;
use empty::EmptyArchitect;
use rooms::RoomsArchitect;
use drunkard::DrunkardArchitect;
use themes::DungeonTheme;
use themes::ForestTheme;
use themes::BeachTheme;

// legion resources have to be thread safe (thus constrained by send + sync)
pub trait MapTheme: Send + Sync {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

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
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    fn new() -> Self {
        Self {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: DungeonTheme::new(),
        }
    }

    pub fn build(rng: &mut RandomNumberGenerator, options: &GameOptions) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(RoomsArchitect{}),
            1 => Box::new(CellularAutomataArchitect{}),
            2 => Box::new(DrunkardArchitect{}),
            _ => Box::new(EmptyArchitect{})
        };

        let mut builder = architect.new(rng, options);

        apply_prefab(&mut builder, rng, &FORTRESS);
        apply_prefab(&mut builder, rng, &SPIRALL);

        builder.theme = match rng.range(0, 3) {
            0 => DungeonTheme::new(),
            1 => BeachTheme::new(),
            _ => ForestTheme::new(),
        };

        builder
    }

    /// collect locations of possible spawn locations
    /// on the map for monsters which are not
    /// too close to the player
    pub fn spawn_locations(&self, start: Point, options: &GameOptions, rng: &mut RandomNumberGenerator, num_monsters: usize) -> Vec<Point> {
        let mut spawn_locations: Vec<Point> = self.map.points_further_than(start, (options.player_fov + 2) as f32);
        let mut spawns: Vec<Point> = Vec::new();

        for _ in 0..num_monsters {
            let target_index = rng.random_slice_index(&spawn_locations).unwrap();
            spawns.push(spawn_locations[target_index].clone());
            spawn_locations.remove(target_index);
        }

        spawns
    }
}
