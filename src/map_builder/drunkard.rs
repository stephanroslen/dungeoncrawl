use super::MapArchitect;
use crate::prelude::*;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
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
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(center, rng, &mut mb.map);

        loop {
            self.drunkard(
                Point::new(
                    rng.range(1, SCREEN_WIDTH - 1),
                    rng.range(1, SCREEN_HEIGHT - 1),
                ),
                rng,
                &mut mb.map,
            );
            if Self::map_completed(&mb.map) {
                mb.fill_unreachable(center);
                if Self::map_completed(&mb.map) {
                    break;
                }
            }
        }
        mb.entity_spawns = mb.spawn_monsters(center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardsWalkArchitect {
    const STAGGER_DISTANCE: usize = 400;
    const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_HEIGHT) as usize;
    const DESIRED_FLOOR: usize = Self::NUM_TILES / 2;

    fn map_completed(map: &Map) -> bool {
        map.tiles.iter().filter(|t| **t == TileType::Floor).count() >= Self::DESIRED_FLOOR
    }

    fn in_inner_bounds(point: Point) -> bool {
        point.x >= 1 && point.x <= SCREEN_WIDTH - 2 && point.y >= 1 && point.y <= SCREEN_HEIGHT - 2
    }
    fn drunkard(&mut self, start: Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            drunkard_pos += match rng.range(0, 4) {
                0 => Point::new(1, 0),
                1 => Point::new(0, 1),
                2 => Point::new(-1, 0),
                _ => Point::new(0, -1),
            };

            if !Self::in_inner_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > Self::STAGGER_DISTANCE {
                break;
            }
        }
    }
}
