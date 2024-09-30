use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            entity_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: random_theme(rng),
        };
        mb.fill(TileType::Wall);
        Self::random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            Self::iteration(&mut mb.map);
        }
        let start = Self::find_start(&mb.map);
        mb.fill_unreachable(start);
        mb.entity_spawns = mb.spawn_monsters(start, rng);
        prefab::apply_prefab(&mut mb, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(rng: &mut RandomNumberGenerator, map: &mut Map) {
        for iy in 1..SCREEN_HEIGHT - 1 {
            for ix in 1..SCREEN_WIDTH - 1 {
                let point = Point::new(ix, iy);
                let idx = map.point2d_to_index(point);
                let roll = rng.range(0, 100);
                map.tiles[idx] = if roll > 55 {
                    TileType::Floor
                } else {
                    TileType::Wall
                }
            }
        }
    }

    fn count_neighbors(point: Point, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                let offset = Point::new(ix, iy);
                let new_point = point + offset;
                if point != new_point && map.tiles[Map::map_idx(new_point)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let point = Point::new(x, y);
                let neighbors = Self::count_neighbors(point, map);
                new_tiles[Map::map_idx(point)] = if neighbors > 4 || neighbors == 0 {
                    TileType::Wall
                } else {
                    TileType::Floor
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        map.index_to_point2d(
            map.tiles
                .iter()
                .enumerate()
                .filter(|(_, t)| **t == TileType::Floor)
                .map(|(idx, _)| {
                    (
                        idx,
                        DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                    )
                })
                .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
                .map(|(idx, _)| idx)
                .unwrap(),
        )
    }
}
