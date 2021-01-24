// this module is private to systems
mod player_input;
mod map_render;
mod entity_render;
mod random_move;
mod chasing;
mod combat;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod item;

// prelude cannot include it since we made nothing public
use crate::prelude::*;

// Now we build different schedule systems to isolate which systems run when

/// While waiting for input, the map and entities still need to be rendered
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

/// If a player is moving we want to detect collisions, flush and render them, and finally check end turn state
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(item::item_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

/// monsters have the random move system, check collisions and also end their turns
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(item::item_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
