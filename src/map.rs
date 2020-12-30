// use our own prelude
use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor(char),
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

/// The map will use row-first encoding since we have a single dimension vector
/// indexing for x and y will look like:
/// ```rust
/// let idx = (y * WIDTH) + x
/// ```
/// the reciprocal (inverse) is calculated by:
/// ```rust
/// let x = index % WIDTH;
/// let y = index / WIDTH; // int division rounds down
/// ```
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor('.'); NUM_TILES],
        }
    }

    pub fn set_tile(&mut self, point: Point, tile: TileType) {
        if let Some(idx) = self.try_idx(point) {
            self.tiles[idx] = tile;
        }
    }

    pub fn fill(&mut self, tile: TileType) {
        self.tiles.iter_mut().for_each(|t| *t = tile);
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        if let Some(idx) = self.try_idx(point) {
            return match self.tiles[idx] {
                TileType::Wall => false,
                TileType::Floor(_) => true
            }
        }

        false
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(map_idx(point.x, point.y))
        } else {
            None
        }
    }
}
