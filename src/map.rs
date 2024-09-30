use crate::map::Revealed::Unrevealed;
use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Revealed {
    Unrevealed,
    Seen,
    FromMap,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<Revealed>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![Unrevealed; NUM_TILES],
        }
    }

    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Self::in_bounds(point) && self.tiles[Self::map_idx(point)] == TileType::Floor
    }

    pub fn map_idx(point: Point) -> usize {
        ((point.y * SCREEN_WIDTH) + point.x) as usize
    }

    pub fn try_idx(point: Point) -> Option<usize> {
        if !Self::in_bounds(point) {
            None
        } else {
            Some(Self::map_idx(point))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if Self::in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(target_idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((target_idx, 1.0));
        }
        if let Some(target_idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((target_idx, 1.0));
        }
        if let Some(target_idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((target_idx, 1.0));
        }
        if let Some(target_idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((target_idx, 1.0));
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        Self::in_bounds(pos)
    }
}
