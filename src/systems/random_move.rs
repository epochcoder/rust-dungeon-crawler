use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovesRandomly)]
pub fn random_move(
    ecs:  &SubWorld,
    commands: &mut CommandBuffer
) {
    let mut movers = <(Entity, &Point, &MovesRandomly)>::query();
    movers.iter(ecs)
        .for_each(|(monster, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let dest = *pos + match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };

            commands.push(((), WantsToMove {
                entity: *monster,
                destination: dest
            }));
        });
}
