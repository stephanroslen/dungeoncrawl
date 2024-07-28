use crate::prelude::*;

mod collision;
mod entity_renders;
mod map_render;
mod player_input;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collision::collisions_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_renders::entity_render_system())
        .build()
}
