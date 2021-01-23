use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Player)]
#[read_component(ChasingPlayer)]
pub fn chasing(
    ecs: &SubWorld,
    #[resource] map: &Map,
    commands: &mut CommandBuffer
) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs)
        .nth(0)
        .expect("Could not find player")
        .0;

    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        1024.0
    );

    movers.iter(ecs).for_each(|(monster, pos, _)| {
        let idx = map_idx(pos.x, pos.y);
        if let Some(dest) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

            // diagonal tile distance is approximately 1.4
            let dest = if distance < 1.4 {
                // move into player
                *player_pos
            } else {
                // move towards player
                map.index_to_point2d(dest)
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
        }
    });
}
