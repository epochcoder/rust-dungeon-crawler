use crate::prelude::*;

const MAX_PLACEMENT_ATTEMPTS: i32 = 10;

/// A fortress, m for monster, - for space and # for a wall
pub const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
-EM------ME-
-###----###-
---#----#---
---#----#---
---######---
------------
", 12, 11);

pub const SPIRALL: (&str, i32, i32) = ("
-###############-------
-#----------M--#-------
-#-###########-#-------
-#-#--------M#-#-----
-#-#-#########-#-----
-#-#---M-------#--
-#-#############--
-#-------------E--
-###############--
", 18, 9);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator, vault: &(&str, i32, i32)) {
    let mut placement = None;

    let dijksta_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < MAX_PLACEMENT_ATTEMPTS {
        let random_rect = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - vault.1),
            rng.range(0, SCREEN_HEIGHT - vault.2),
            vault.1,
            vault.2
        );

        let mut can_place = false;

        // find entrances in the prefab and check
        // if they are reachable, rather than checking all points.
        let vault_chars: Vec<char> = vault.0.chars()
            .collect();

        let mut x = random_rect.x1;
        let mut y = random_rect.y1;

        // can we do this in a better way? maybe a mini indexed map for the piece
        vault_chars.iter().for_each(|c| {
            match c {
                'E' => {
                    let map_location = Point::new(x, y);
                    // figure out if the entrance is reachable
                    let idx = mb.map.point2d_to_index(map_location);
                    let dist = dijksta_map.map[idx];

                    let reachable = dist < 2000.0 && dist > 20.0;
                    if reachable && mb.amulet_start != map_location {
                        //println!("Found entrance at point: {:?}", map_location);
                        can_place = true;
                    }
                },
                '\n' => {
                    x = random_rect.x1;
                    y += 1;
                },
                _ => { }
            }

            x += 1;
        });

        if can_place {
            placement = Some(Point::new(random_rect.x1, random_rect.y1));
            let vault_points = random_rect.point_set();

            // ensure we do not overwrite monsters
            mb.monster_spawns.retain(|pt| !vault_points.contains(pt));
        }

        attempts += 1;
    }

    if let Some(pt) = placement {
        //println!("Placing vault at: {:?}", pt);

        let mut x = pt.x;
        let mut y = pt.y;

        vault.0.chars().into_iter().for_each(|c| {
            let delta = Point::new(x, y);
            match c {
                '-' | 'E' => mb.map.set_tile(delta, TileType::Floor),
                '#' => mb.map.set_tile(delta, TileType::Wall),
                'M' => {
                    // might have been another tile
                    mb.map.set_tile(delta, TileType::Floor);
                    mb.monster_spawns.push(delta);
                },
                '\n' => {
                    x = pt.x;
                    y += 1;
                },
                //'E' => mb.map.set_tile(delta, TileType::Test),
                _ => println!("Cannot place prefab piece: {:?}", c)
            }

            x += 1;
        });
    }
}

