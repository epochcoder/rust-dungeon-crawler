use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Name)]
#[read_component(MovesRandomly)]
pub fn random_move(
    ecs:  &SubWorld,
    commands: &mut CommandBuffer
) {
    let mut movers = <(Entity, &Point, &MovesRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    movers.iter(ecs)
        .for_each(|(monster, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let dest = *pos + match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };

            let mut attacked = false;
            positions.iter(ecs)
                .filter(|(_, pos, _)| **pos == dest)
                .for_each(|(victim, _, _)| {
                    // we only want to attack the player for now
                    if ecs.entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        attacked = true;

                        println!("monster {} attacks player!", &ecs
                            .entry_ref(*monster)
                            .unwrap()
                            .get_component::<Name>()
                            .unwrap().0);

                        commands.push(((), WantsToAttack {
                            victim: *victim,
                            attacker: *monster
                        }));
                    }
                });

            if !attacked {
                commands.push(((), WantsToMove {
                    entity: *monster,
                    destination: dest
                }));
            }
        });
}
