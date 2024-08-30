use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let (player_fov, player_pos) = <(&FieldOfView, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if Map::in_bounds(point) {
                let tile_visible = player_fov.visible_tiles.contains(&point);
                let idx = Map::map_idx(point);
                if tile_visible || map.revealed_tiles[idx] {
                    let dist = DistanceAlg::Pythagoras.distance2d(*player_pos, point);
                    let tint_scale = tint_scale_calc(
                        if tile_visible { Some(dist) } else { None },
                        player_fov.radius as f32,
                    );
                    let tint = RGB::from_f32(tint_scale, tint_scale, tint_scale);
                    let glyph = match map.tiles[idx] {
                        TileType::Floor => to_cp437('.'),
                        TileType::Wall => to_cp437('#'),
                    };
                    draw_batch.set(point - offset, ColorPair::new(tint, BLACK), glyph);
                }
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
