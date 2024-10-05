use crate::prelude::*;
use std::collections::BTreeMap;
use std::iter::Iterator;

#[system]
#[read_component(Carried)]
#[read_component(Equipped)]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let (player_entity, player_health, map_level) = <(Entity, &Health, &Player)>::query()
        .iter(ecs)
        .next()
        .map(|(entity, health, player)| (*entity, health, player.map_level))
        .unwrap();

    let mut carried: BTreeMap<String, usize> = BTreeMap::new();
    <(&Name, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, carried)| carried.by == player_entity)
        .for_each(|(name, _)| {
            if let Some(entry) = carried.get_mut(&name.name) {
                *entry += 1;
            } else {
                carried.insert(name.name.clone(), 1);
            }
        });

    let mut y = 3;
    for (item, count) in carried {
        draw_batch.print(
            Point::new(3, y),
            format!("{} : {} x {}", y - 2, count, item),
        );
        y += 1;
    }
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    y = 3;
    <(&Name, &Equipped)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, equipped)| equipped.by == player_entity)
        .for_each(|(name, _)| {
            draw_batch.print_right(Point::new(SCREEN_WIDTH * 2 - 3, y), &name.name);
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH * 2 - 3, 2),
            "Items equipped",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.print_centered(
        0,
        "Explore the Dungeon. Cursor keys to move. G to pick up item and number to use it.",
    );
    draw_batch.bar_horizontal(
        Point::new(0, SCREEN_HEIGHT * 2 - 1),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        SCREEN_HEIGHT * 2 - 1,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2 - 2),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    draw_batch.submit(10000).expect("Batch error");
}
