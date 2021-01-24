use crate::prelude::*;

#[system]
#[write_component(Health)]
#[read_component(Point)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Player)] // request read access to the player marker type
pub fn player_input(
    ecs: &mut SubWorld, // only contains the requested components
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>, // TODO: how to do multiple keys?
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::zero(),
        };

        let mut did_something = false;

        // we just request all entities with a point component
        // that have the player component marker
        let mut players = <(Entity, &Point)>::query()
            // not an iterator yet! (until iter() gets called)
            // filter requires the component to exists, but can't use its content
            .filter(component::<Player>());

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        if delta.x != 0 || delta.y != 0 {
            // now get monsters to see if player will move into one
            let mut enemies = <(Entity, &Point)>::query()
                .filter(component::<Enemy>());

            let mut hit_enemy = false;
            // look for any monsters we might have moved into
            enemies
                .iter_mut(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(enemy, _)| {
                    hit_enemy = true;
                    did_something = true;

                    // we would move into a monster now
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *enemy,
                        },
                    ));
                });

            let mut items = <(Entity, &Point)>::query()
                .filter(component::<Item>());

            items.iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(item, _)| {
                    did_something = true;

                    commands.push((
                        (),
                        ItemReceived {
                            receiver: player_entity,
                            item: *item,
                        }
                    ));
                });

            // check if we hit an item, apply item effects, or end game

            if !hit_enemy {
                did_something = true;
                // send an entity/message that we intent to move
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        if !did_something {
            // lets give the player some health for waiting a turn
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        // end player turn
        *turn_state = TurnState::PlayerTurn;
    }
}
