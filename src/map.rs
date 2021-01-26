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
    pub revealed_tiles: Vec<bool>,
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
            revealed_tiles: vec![false; NUM_TILES],
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

    pub fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let dest = loc + delta;

        if self.can_enter_tile(dest) {
            Some(self.point2d_to_index(dest))
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) ->Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        // cannot see through wallsx    x
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2)
            )
    }
}
