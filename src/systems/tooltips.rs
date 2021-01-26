use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] mouse_pos: &Point
) {
    // get player field of view
    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    // including the entity here, includes the entity that owns the components
    let mut positions = <(Entity, &Point, &Name)>::query();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);

    positions
        .iter(ecs)
        // filter for the mouse hovering exactly above the monster
        .filter(|(_, pos, _)|
            **pos == map_pos
                && player_fov.visible_tiles.contains(&pos))
        .for_each(|(ent, _, name)| {
            let mut screen_pos = *mouse_pos * 4;
            screen_pos.y = screen_pos.y - 1;

            // we don't query for the health component in positions,
            // since we might support other tooltips also
            let display = if let Ok(health) = ecs
                .entry_ref(*ent)
                .unwrap()
                .get_component::<Health>()
            {
                format!("{} : {} hp", &name.0, health.current)
            } else {
                name.0.clone()
            };

            draw_batch.print(screen_pos, display);
        });

    draw_batch.submit(110000).expect("Error rendering tooltips");
}
