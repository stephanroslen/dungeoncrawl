use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let player_pos = <(&Point, &Player)>::query().iter(ecs).nth(0).unwrap().0;
    let player_idx = Map::map_idx(*player_pos);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    <(Entity, &Point, &FieldOfView)>::query()
        .filter(component::<ChasingPlayer>())
        .iter(ecs)
        .filter(|(_, _, fov)| fov.visible_tiles.contains(&player_pos))
        .for_each(|(entity, pos, _)| {
            let idx = Map::map_idx(*pos);
            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
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
