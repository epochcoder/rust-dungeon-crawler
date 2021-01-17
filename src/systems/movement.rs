use crate::prelude::*;

/// The movement system iterates all entities with a WantsToMove component.
/// It then checks that the move is valid, and if it is replaces the Point
/// component of the target entity. If the entity is a player, it also updates the camera.”
// derive the query parameters from the arguments (only for single queries
#[system(for_each)]
// these provide entities containing components to the SubWorld
#[read_component(Player)]
pub fn movement(
    message_entity: &Entity,
    movement_intention: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if map.can_enter_tile(movement_intention.destination) {
        // rather use commands as it is safer than to modify entities directly
        // add/replace the point on the destination entity
        commands.add_component(movement_intention.entity, movement_intention.destination);

        // look up the entity that wants to move (in this case, a player)
        if ecs.entry_ref(movement_intention.entity)
                .unwrap().get_component::<Player>().is_ok() {
            camera.on_player_move(movement_intention.destination);
        }
    }

    // mark the message/entity as processed so we don't do it again
    commands.remove(*message_entity);
}
