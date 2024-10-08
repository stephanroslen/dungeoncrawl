use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Revealed {
    Unrevealable,
    Unrevealed,
    Seen,
    FromMap,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<Revealed>,
    pub dijkstra_maps: Vec<Option<DijkstraMap>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![Revealed::Unrevealable; NUM_TILES],
            dijkstra_maps: (0..NUM_TILES).map(|_| None).collect(),
        }
    }

    pub fn update_revealability(&mut self) {
        let tiles = &self.tiles;
        let revealed = &mut self.revealed_tiles;
        tiles.iter().enumerate().for_each(|(idx, tile)| {
            if !Self::is_opaque(*tile) {
                let point_for_idx = Self::map_point(idx);
                let offsets = [
                    Point::new(-1, -1),
                    Point::new(0, -1),
                    Point::new(1, -1),
                    Point::new(-1, 0),
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(-1, 1),
                    Point::new(0, 1),
                    Point::new(1, 1),
                ];
                for offset in offsets {
                    let point = point_for_idx + offset;
                    let idx_for_point = Self::map_idx(point);
                    revealed[idx_for_point] = Revealed::Unrevealed;
                }
            }
        });
    }

    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        let tile_type = self.tiles[Self::map_idx(point)];
        Self::in_bounds(point) && [TileType::Floor, TileType::Exit].contains(&tile_type)
    }

    pub fn map_idx(point: Point) -> usize {
        ((point.y * SCREEN_WIDTH) + point.x) as usize
    }

    pub fn map_point(idx: usize) -> Point {
        Point::new(idx % SCREEN_WIDTH as usize, idx / SCREEN_WIDTH as usize)
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

    fn is_opaque(tile_type: TileType) -> bool {
        tile_type == TileType::Wall
    }

    pub fn update_dijkstra_maps(&mut self) {
        let tiles = &self.tiles;
        let mut dijkstra_maps: Vec<Option<DijkstraMap>> = (0..NUM_TILES).map(|_| None).collect();
        tiles.iter().enumerate().for_each(|(idx, _)| {
            if self.can_enter_tile(Self::map_point(idx)) {
                let target = vec![idx];
                dijkstra_maps[idx] = Some(DijkstraMap::new(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    &target,
                    self,
                    1024.0,
                ));
            };
        });
        self.dijkstra_maps = dijkstra_maps;
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
        Self::is_opaque(self.tiles[idx])
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
