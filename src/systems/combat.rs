use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    // look for attack messages
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(attack_entity, attack)| (*attack_entity, attack.victim))
        .collect();

    victims.iter().for_each(|( message, victim)| {
        // look for health component
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 {
                // killed them
                commands.remove(*victim)
            }
        }

        commands.remove(*message);
    });
}
