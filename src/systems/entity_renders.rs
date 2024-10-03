use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let (player_fov, player_pos) = <(&FieldOfView, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            let dist = DistanceAlg::Pythagoras.distance2d(*player_pos, *pos);
            let tint_scale = tint_scale_calc(Some(dist), player_fov.radius as f32);
            draw_batch.set(
                *pos - offset,
                tint_colorpair_conversion(render.color, tint_scale),
                render.glyph,
            );
        });
    draw_batch.submit(5000).expect("Batch error");
}
