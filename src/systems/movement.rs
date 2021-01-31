use crate::prelude::*;

/// The movement system iterates all entities with a WantsToMove component.
/// It then checks that the move is valid, and if it is replaces the Point
/// component of the target entity. If the entity is a player, it also updates the camera.”
// derive the query parameters from the arguments (only for single queries
#[system(for_each)]
// these provide entities containing components to the SubWorld
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    message_entity: &Entity,
    movement_intention: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(movement_intention.destination) {
        if let Ok(entry) = ecs.entry_ref(movement_intention.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                // since we moved, clone a dirty new fov, for fov system to rebuild it
                commands.add_component(movement_intention.entity, fov.clone_dirty());

                // rather use commands as it is safer than to modify entities directly
                // add/replace the point on the destination entity
                commands.add_component(movement_intention.entity, movement_intention.destination);

                // look up the entity that wants to move (in this case, a player)
                if ecs
                    .entry_ref(movement_intention.entity)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    camera.on_player_move(movement_intention.destination);

                    // updated map tiles as visible (spatial memory of player)
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    })
                }
            }
        }
    }

    // mark the message/entity as processed so we don't do it again
    commands.remove(*message_entity);
}
