use super::MapArchitect;
use crate::prelude::*;
use std::cmp::{min, max};

pub struct RoomsArchitect;

impl RoomsArchitect {
    fn build_random_rooms(builder: &mut MapBuilder, rng: &mut RandomNumberGenerator, options: &GameOptions) {
        while builder.rooms.len() < options.max_rooms {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(3, options.room_size),
                rng.range(3, options.room_size),
            );

            let mut overlap = false;
            for r in builder.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                // call closure for each point in the room (x, y)
                room.for_each(|p| {
                    builder.map.set_tile(p, TileType::Floor);
                });

                builder.rooms.push(room);
            }
        }
    }

    fn apply_tunnel(builder: &mut MapBuilder, range: (i32, i32), pinned: i32, dir: TunnelType) {
        for i in min(range.0, range.1)..=max(range.0, range.1) {
            builder.map.set_tile(
                match dir {
                    TunnelType::Horizontal => Point::new(i, pinned),
                    TunnelType::Vertical => Point::new(pinned, i),
                },
                TileType::Floor,
            );
        }
    }

    fn apply_vertical_tunnel(builder: &mut MapBuilder, range: (i32, i32), x: i32) {
        RoomsArchitect::apply_tunnel(builder,range, x, TunnelType::Vertical);
    }

    fn apply_horizontal_tunnel(builder: &mut MapBuilder, range: (i32, i32), y: i32) {
        RoomsArchitect::apply_tunnel(builder, range, y, TunnelType::Horizontal);
    }

    fn build_corridors(builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        let mut rooms = builder.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 3 {
                RoomsArchitect::apply_vertical_tunnel(builder, (prev.y, new.y), new.x);
                RoomsArchitect::apply_horizontal_tunnel(builder, (prev.x, new.x), prev.y);
            } else {
                RoomsArchitect::apply_vertical_tunnel(builder, (prev.y, new.y), prev.x);
                RoomsArchitect::apply_horizontal_tunnel(builder, (prev.x, new.x), new.y);
            }
        }
    }
}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator, options: &GameOptions) -> MapBuilder {
        println!("Running RoomsArchitect");
        let mut builder = MapBuilder::new();

        builder.map.fill(TileType::Wall);

        RoomsArchitect::build_random_rooms(&mut builder, rng, options);
        RoomsArchitect::build_corridors(&mut builder, rng);

        builder.player_start = builder.rooms[0].center();
        builder.amulet_start = builder.map.find_most_distant_from(builder.player_start);

        // create monsters
        builder.monster_spawns = builder
            .rooms
            .iter()
            .skip(1) // we are in the first room
            .map(|r| {
                let x = &rng.range(r.x1, r.x2);
                let y = &rng.range(r.y1, r.y2);
                Point::new(*x, *y)
            })
            .collect::<Vec<Point>>();

        builder
    }
}
