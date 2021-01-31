use crate::prelude::*;

/// Push a 'player' entity onto the world, represented as a tuple of components
pub fn spawn_player(ecs: &mut World, camera: &mut Camera, position: Point, fov: i32) {
    camera.on_player_move(position);
    ecs.push((
        Player,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(fov),
    ));
}

/// Push a 'monster' entity onto the world, represented as a tuple of different components
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point, fov: i32) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        ChasingPlayer,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph
            // glyph: match rng.range(0, 4) {
            //     0 => to_cp437('E'), // ettin
            //     1 => to_cp437('O'), // ogre
            //     2 => to_cp437('o'), // orc
            //     _ => to_cp437('g'), // goblin
            // },
        },
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(fov),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
