use crate::prelude::*;

const MAX_PLACEMENT_ATTEMPTS: i32 = 10;

/// A fortress, m for monster, - for space and # for a wall
pub const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
", 12, 11);

pub const SPIRALL: (&str, i32, i32) = ("
-############-----
-#-----M--#-------
-#-######-#-------
-#-#--MM#-#-------
-#-#-####-#-------
-#-#---M--#-------
-#-#############--
-#----------------
-################-
", 19, 8);

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

        // TODO: this condition not be inverted? in book it is opposite, but we need to tes every point...
        let mut can_place = true;
        random_rect.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let dist = dijksta_map.map[idx];
            let reachable = dist < 2000.0 && dist > 20.0;
            if !reachable || mb.amulet_start == pt {
                can_place = false;
            }
        });

        if can_place {
            placement = Some(Point::new(random_rect.x1, random_rect.y1));
            let points = random_rect.point_set();
            // ensure we do not overwrite monsters
            mb.monster_spawns.retain(|pt| !points.contains(pt));
        }

        attempts += 1;
    }

    if let Some(pt) = placement {
        println!("Placing vault at: {:?}", pt);

        let mut x = pt.x;
        let mut y = pt.y;

        vault.0.chars().into_iter().for_each(|c| {
            let delta = Point::new(x, y);
            match c {
                '-' => mb.map.set_tile(delta, TileType::Floor),
                '#' => mb.map.set_tile(delta, TileType::Wall),
                'M' => mb.monster_spawns.push(delta),
                '\n' => {
                    x = pt.x;
                    y += 1;
                }
                _ => println!("Cannot place prefab piece: {:?}", c)
            }

            x += 1;
        });
    }
}

