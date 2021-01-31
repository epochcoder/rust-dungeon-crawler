use crate::prelude::*;

#[system]
#[write_component(FieldOfView)]
#[read_component(Point)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut views = <(&Point, &mut FieldOfView)>::query();
    views
        .iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.is_dirty = false;
        });

    // fov can be used for many things, ranged combat, explosions, fog of war, etc..
}
