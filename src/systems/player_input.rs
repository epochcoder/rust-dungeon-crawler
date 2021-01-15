use crate::prelude::*;

#[system]
#[write_component(Point)] // request writable access to the point component of the entity
#[read_component(Player)] // request read access to the player marker type
pub fn player_input(
    ecs: &mut SubWorld, // only contains the requested components
    #[resource] map: &Map, // requests access to in legion's resource handler
    #[resource] key: &Option<VirtualKeyCode>, // TODO: how to do multiple keys?
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::zero()
        };

        if delta.x != 0 || delta.y != 0 {
            // we just request all entities with a point component
            // that have the player component marker
            let mut players = <&mut Point>::query()
                // not an iterator yet! (until iter() gets called)
                // filter requires the component to exists, but can't use its content
                .filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);

                    // end player turn
                    *turn_state = TurnState::PlayerTurn;
                }
            })
        }
    }
}
