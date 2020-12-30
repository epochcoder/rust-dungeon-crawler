use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(point, render)| {
            draw_batch.set(
                *point - offset,
                render.color,
                render.glyph,
            );
        });

    draw_batch.submit(5000)
        .expect("Could new submit entity_render batch");
}
