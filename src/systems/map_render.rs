use crate::prelude::*;

#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);


    let offset = Point::new(camera.left_x, camera.top_y);
    for y in offset.y ..= camera.bottom_y {
        for x in offset.x ..= camera.right_x {
            let pt = Point::new(x, y);

            if map.in_bounds(pt) {
                let idx = map_idx(pt.x, pt.y);
                let (glyph, color) = match map.tiles[idx] {
                    TileType::Wall => (to_cp437('#'), GREEN),
                    TileType::Floor(_) => (to_cp437('.'), YELLOW)
                };

                draw_batch.set(
                    pt - offset,
                    ColorPair::new(color,  BLACK),
                    glyph
                );
            }
        }
    }

    //only submit after all drawing operations have been completed
    draw_batch.submit(0)
        .expect("Error submitting map_render batch");
}
