use crate::prelude::*;
use std::collections::BTreeMap;

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .map(|entity| *entity)
        .unwrap();

    let mut some_carried_entry: BTreeMap<String, Entity> = BTreeMap::new();
    <(Entity, &Name, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, _, carried)| carried.by == player_entity)
        .for_each(|(entity, name, _)| {
            some_carried_entry.insert(name.name.clone(), *entity);
        });

    let item_entity = some_carried_entry.iter().nth(n).map(|(_, entity)| *entity);

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::new(0, 0)
}

#[system]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Point)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .map(|(entity, pos)| (*entity, *pos))
                    .next()
                    .unwrap();
                <(Entity, &Point)>::query()
                    .filter(component::<Item>())
                    .iter(ecs)
                    .filter(|(_, &item_pos)| item_pos == player_pos)
                    .for_each(|(entity, _)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried { by: player });

                        let entity_ref = ecs.entry_ref(*entity).unwrap();
                        if entity_ref.get_component::<Weapon>().is_ok() {
                            <(Entity, &Carried)>::query()
                                .filter(component::<Weapon>())
                                .iter(ecs)
                                .filter(|(_, c)| c.by == player)
                                .for_each(|(entity, _)| {
                                    commands.remove(*entity);
                                })
                        }
                    });
                Point::new(0, 0)
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .map(|(entity, pos)| (*entity, *pos + delta))
            .next()
            .unwrap();

        if delta != Point::zero() {
            let mut hit_something = false;
            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
