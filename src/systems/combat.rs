use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Equipped)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let victims: Vec<(Entity, Entity, Entity)> = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.damage
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Equipped, &Damage)>::query()
            .iter(ecs)
            .filter(|(equipped, _)| equipped.by == *attacker)
            .map(|(_, dmg)| dmg.damage)
            .sum();

        let final_damage = base_damage + weapon_damage;

        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}
