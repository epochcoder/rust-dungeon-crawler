// this module is private to systems
mod player_input;
mod map_render;
mod entity_render;

// prelude cannot include it since we made nothing public
use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}
