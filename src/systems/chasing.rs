use crate::prelude::*;

fn sample_lowest_exit(
    rng: &mut RandomNumberGenerator,
    dm: &DijkstraMap,
    position: usize,
    map: &dyn BaseMap,
) -> Option<usize> {
    let mut exits = map.get_available_exits(position);

    if exits.is_empty() {
        return None;
    }

    exits.sort_by(|a, b| {
        dm.map[a.0 as usize]
            .partial_cmp(&dm.map[b.0 as usize])
            .unwrap()
    });

    let options: Vec<usize> = exits
        .iter()
        .filter(|(idx, _)| dm.map[*idx] == dm.map[exits[0].0])
        .map(|(idx, _)| *idx as usize)
        .collect();

    let rsi = rng.random_slice_index(&options).unwrap();
    Some(options[rsi])
}

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(ChasingPlayer)]
pub fn chasing(#[resource] map: &Map, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();

    let player_pos = *<(&Point, &Player)>::query().iter(ecs).nth(0).unwrap().0;
    let player_idx = Map::map_idx(player_pos);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    <(&Point, &FieldOfView, &mut ChasingPlayer)>::query()
        .iter_mut(ecs)
        .for_each(|(pos, fov, chasing_player)| {
            if fov.visible_tiles.contains(&player_pos) {
                chasing_player.expecting_player_at = Some(player_pos);
            }
            if Some(*pos) == chasing_player.expecting_player_at {
                chasing_player.expecting_player_at = None;
            }
        });

    <(Entity, &Point, &ChasingPlayer)>::query()
        .iter(ecs)
        .filter(|(_, _, chasing_player)| chasing_player.expecting_player_at.is_some())
        .for_each(|(entity, pos, chasing_player)| {
            let expecting_player_at = chasing_player.expecting_player_at.unwrap();
            let idx = Map::map_idx(*pos);
            if let Some(destination) = sample_lowest_exit(&mut rng, &dijkstra_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, player_pos);
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    expecting_player_at
                };

                let mut attacked = false;
                <(Entity, &Point)>::query()
                    .filter(component::<Health>())
                    .iter(ecs)
                    .filter(|(_, target_pos)| **target_pos == destination)
                    .for_each(|(victim, _)| {
                        if ecs
                            .entry_ref(*victim)
                            .unwrap()
                            .get_component::<Player>()
                            .is_ok()
                        {
                            commands.push((
                                (),
                                WantsToAttack {
                                    attacker: *entity,
                                    victim: *victim,
                                },
                            ));
                            attacked = true;
                        }
                    });

                if !attacked {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                }
            }
        })
}
