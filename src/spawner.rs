use crate::prelude::*;

/// Push a 'player' entity onto the world, represented as a tuple of components
pub fn spawn_player(ecs: &mut World, camera: &mut Camera, position: Point) {
    camera.on_player_move(position);
    ecs.push((
        Player,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        }
    ));
}

/// Push a 'monster' entity onto the world, represented as a tuple of different components
pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point) {

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(RED, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'), // ettin
                1 => to_cp437('O'), // ogre
                2 => to_cp437('o'), // orc
                _ => to_cp437('g'), // goblin
            },
        }
    ));
}
