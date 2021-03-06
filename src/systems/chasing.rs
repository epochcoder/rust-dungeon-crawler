use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(FieldOfView)]
#[read_component(ChasingPlayer)]
pub fn chasing(ecs: &SubWorld, #[resource] map: &Map, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).nth(0).expect("Could not find player").0;

    let player_idx = map_idx(player_pos.x, player_pos.y);
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers
        .iter(ecs)
        .filter(|(_, _, _, fov)| fov.visible_tiles.contains(&player_pos))
        .for_each(|(monster, monster_pos, _, _)| {
            let monster_map_idx = map_idx(monster_pos.x, monster_pos.y);

            // find the easiest way to the player
            if let Some(dest) = DijkstraMap::find_lowest_exit(&dijkstra_map, monster_map_idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*monster_pos, *player_pos);

                // diagonal tile distance is approximately 1.4
                let dest = if distance < 1.2 {
                    // move into player
                    *player_pos
                } else {
                    // move towards player
                    map.index_to_point2d(dest)
                };

                let mut attacked = false;
                positions
                    .iter(ecs)
                    .filter(|(_, pos, _)| **pos == dest)
                    .for_each(|(victim, _, _)| {
                        // we only want to attack the player for now
                        if ecs
                            .entry_ref(*victim)
                            .unwrap()
                            .get_component::<Player>()
                            .is_ok()
                        {
                            attacked = true;
                            commands.push((
                                (),
                                WantsToAttack {
                                    victim: *victim,
                                    attacker: *monster,
                                },
                            ));
                        }

                        if !attacked
                            && ecs
                                .entry_ref(*victim)
                                .unwrap()
                                .get_component::<Enemy>()
                                .is_ok()
                        {
                            println!("Will move into another monster!");
                            attacked = true;
                        }
                    });

                if !attacked {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *monster,
                            destination: dest,
                        },
                    ));
                }
            }
        });
}
