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

    exits.sort_by(|a, b| dm.map[a.0].partial_cmp(&dm.map[b.0]).unwrap());

    let options: Vec<usize> = exits
        .iter()
        .filter(|(idx, _)| dm.map[*idx] == dm.map[exits[0].0])
        .map(|(idx, _)| *idx)
        .collect();

    let rsi = rng.random_slice_index(&options).unwrap();
    Some(options[rsi])
}

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(RoamingAndChasingPlayer)]
pub fn roaming_and_chasing(
    #[resource] map: &Map,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut rng = RandomNumberGenerator::new();

    let player_pos = *<(&Point, &Player)>::query().iter(ecs).next().unwrap().0;

    <(&Point, &FieldOfView, &mut RoamingAndChasingPlayer)>::query()
        .iter_mut(ecs)
        .for_each(|(pos, fov, roaming_and_chasing_player)| {
            if fov.visible_tiles.contains(&player_pos) {
                roaming_and_chasing_player.going_to = Some(player_pos);
            }
            if roaming_and_chasing_player.going_to.is_none()
                || Some(*pos) == roaming_and_chasing_player.going_to
            {
                roaming_and_chasing_player.going_to = match rng.range(1, 50) {
                    1 => Some(roaming_and_chasing_player.home_location),
                    2..5 => {
                        let idx = Map::map_idx(*pos);
                        let dijkstra_map = &map.dijkstra_maps[idx];
                        let targets = dijkstra_map
                            .as_ref()
                            .unwrap()
                            .map
                            .iter()
                            .enumerate()
                            .filter(|(_, dist)| **dist < 10.0)
                            .map(|(idx, _)| idx)
                            .collect::<Vec<usize>>();
                        let target_idx = targets[rng.random_slice_index(&targets).unwrap()];
                        Some(Map::map_point(target_idx))
                    }
                    _ => None,
                };
            }
        });

    <(Entity, &Point, &RoamingAndChasingPlayer)>::query()
        .iter(ecs)
        .filter(|(_, _, roaming_and_chasing_player)| roaming_and_chasing_player.going_to.is_some())
        .for_each(|(entity, pos, roaming_and_chasing_player)| {
            if let Some(going_to) = roaming_and_chasing_player.going_to {
                let idx = Map::map_idx(*pos);
                let dijkstra_map = map.dijkstra_maps[Map::map_idx(going_to)].as_ref().unwrap();
                if let Some(destination) = sample_lowest_exit(&mut rng, dijkstra_map, idx, map) {
                    let distance = DistanceAlg::Pythagoras.distance2d(*pos, going_to);
                    let destination = if distance > 1.2 {
                        map.index_to_point2d(destination)
                    } else {
                        going_to
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
            }
        })
}
