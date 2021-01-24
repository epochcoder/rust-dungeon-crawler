use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
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
        // check if the player died
        let is_player = ecs.entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        // look for health component
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 && !is_player {
                // killed them
                commands.remove(*victim)
            }
        }

        commands.remove(*message);
    });
}
