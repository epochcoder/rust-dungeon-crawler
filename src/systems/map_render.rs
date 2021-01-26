use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera
) {
    // get player field of view
    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let offset = Point::new(camera.left_x, camera.top_y);
    for y in offset.y ..= camera.bottom_y {
        for x in offset.x ..= camera.right_x {
            let pt = Point::new(x, y);

            if map.in_bounds(pt) {
                let idx = map_idx(pt.x, pt.y);
                let sees_tile = player_fov.visible_tiles.contains(&pt);
                if map.revealed_tiles[idx] || sees_tile {
                    let tint = if sees_tile {
                        WHITE // has no effect
                    } else {
                        // multiplied by to lower
                        DARK_GRAY
                    };

                    let glyph = match map.tiles[idx] {
                        TileType::Wall => to_cp437('#'),
                        TileType::Floor(_) => to_cp437('.')
                    };

                    draw_batch.set(
                        pt - offset,
                        ColorPair::new(tint, BLACK),
                        glyph
                    );
                }
            }
        }
    }

    //only submit after all drawing operations have been completed
    draw_batch.submit(0)
        .expect("Error submitting map_render batch");
}
