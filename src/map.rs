use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
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
}
