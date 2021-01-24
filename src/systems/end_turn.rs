use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState
) {
    let mut player_hp = <&Health>::query()
        .filter(component::<Player>());

    let mut next_turn = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => *turn_state
    };

    player_hp.iter(ecs)
        .for_each(|hp| {
            if hp.current < 1 {
                next_turn = TurnState::GameOver;
            }
        });

    *turn_state = next_turn;
}
