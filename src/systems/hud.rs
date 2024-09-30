use crate::prelude::*;
use std::iter::Iterator;

#[system]
#[read_component(Carried)]
#[read_component(Health)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let (player_entity, player_health) = <(Entity, &Health)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .map(|(entity, health)| (*entity, health))
        .unwrap();

    let mut y = 3;
    <(&Name, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, carried)| carried.by == player_entity)
        .for_each(|(name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, name.0));
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.submit(10000).expect("Batch error");
}
