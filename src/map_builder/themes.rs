use crate::prelude::*;

pub struct DungeonTheme {}
pub struct ForestTheme {}
pub struct BeachTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

impl BeachTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> u16 {
        match tile_type {
            TileType::Wall => to_cp437('q'),
            TileType::Floor => to_cp437('p'),
            TileType::Test => to_cp437('T'),
        }
    }
}

impl MapTheme for BeachTheme {
    fn tile_to_render(&self, tile_type: TileType) -> u16 {
        match tile_type {
            TileType::Wall => to_cp437('6'),
            TileType::Floor => to_cp437('7'),
            TileType::Test => to_cp437('T'),
        }
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> u16 {
        match tile_type {
            TileType::Wall => to_cp437('"'),
            TileType::Floor => to_cp437(';'),
            TileType::Test => to_cp437('T'),
        }
    }
}
