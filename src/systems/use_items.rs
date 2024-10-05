use crate::prelude::*;
use std::collections::HashSet;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesDepletion)]
#[read_component(ProvidesDungeonMap)]
#[read_component(ProvidesEquipment)]
#[read_component(ProvidesHealing)]
#[read_component(Carried)]
#[read_component(Weapon)]
#[write_component(Health)]
#[read_component(Equipped)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    let mut unequip_weapons_by = HashSet::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item).unwrap();
            let item_depletes = item.get_component::<ProvidesDepletion>().is_ok();

            if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                healing_to_apply.push((activate.used_by, healing.amount));
            }
            if item.get_component::<ProvidesDungeonMap>().is_ok() {
                map.revealed_tiles.iter_mut().for_each(|t| {
                    if *t == Revealed::Unrevealed {
                        *t = Revealed::FromMap
                    }
                });
            }

            if item.get_component::<ProvidesEquipment>().is_ok() {
                if item.get_component::<Weapon>().is_ok() {
                    unequip_weapons_by.insert(activate.used_by);
                }

                commands.remove_component::<Carried>(activate.item);
                commands.add_component(
                    activate.item,
                    Equipped {
                        by: activate.used_by,
                    },
                );
            }

            if item_depletes {
                commands.remove(activate.item);
            }
            commands.remove(*entity);
        });

    for (target, by_health) in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(*target) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + by_health);
            }
        }
    }

    if !unequip_weapons_by.is_empty() {
        <(Entity, &Equipped)>::query()
            .filter(component::<Weapon>())
            .iter(ecs)
            .filter(|(_, equipped)| unequip_weapons_by.contains(&equipped.by))
            .for_each(|(entity, equipped)| {
                let by = equipped.by;
                commands.remove_component::<Equipped>(*entity);
                commands.add_component(*entity, Carried { by });
            });
    }
}
