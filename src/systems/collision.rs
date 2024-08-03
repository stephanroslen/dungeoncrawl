use crate::prelude::*;

#[system]
#[read_component(Point)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| player_pos = *pos);

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
