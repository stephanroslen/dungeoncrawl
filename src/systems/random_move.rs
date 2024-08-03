use crate::prelude::*;

#[system]
#[write_component(Point)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &mut Point)>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|(entity, pos)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
}
