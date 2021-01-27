use std::cmp::{max, min};

use crate::prelude::*;

pub enum TunnelType {
    Horizontal,
    Vertical,
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point
}

impl MapBuilder {
    pub fn build(rng: &mut RandomNumberGenerator, options: &GameOptions) -> Self {
        let mut builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // cave carving mode
        builder.map.fill(TileType::Wall);
        builder.build_random_rooms(rng, options);
        builder.build_corridors(rng);
        builder.player_start = builder.rooms[0].center();

        // create a dijstra map to put the amulet as far as we can from the player
        let p_idx = builder.map.point2d_to_index(builder.player_start);
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![p_idx],
            &builder.map,
            1024.0
        );

        //find the highest path in the map
        const UNREACHABLE: f32 = f32::MAX;
        builder.amulet_start = builder.map.index_to_point2d(
            dijkstra_map.map
            .iter()
            .enumerate()
            .filter(|(_, dist)| **dist < UNREACHABLE)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap_or((p_idx, &0f32))
            .0
        );

        builder
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator, options: &GameOptions) {
        while self.rooms.len() < options.max_rooms {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(3, options.room_size),
                rng.range(3, options.room_size),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                // call closure for each point in the room (x, y)
                room.for_each(|p| {
                    self.map.set_tile(p, TileType::Floor('.'));
                });

                self.rooms.push(room);
            }
        }
    }

    fn apply_tunnel(&mut self, range: (i32, i32), pinned: i32, dir: TunnelType) {
        for i in min(range.0, range.1)..=max(range.0, range.1) {
            self.map.set_tile(match dir {
                TunnelType::Horizontal => Point::new(i, pinned),
                TunnelType::Vertical => Point::new(pinned, i),
            }, TileType::Floor('.'));
        }
    }

    fn apply_vertical_tunnel(&mut self, range: (i32, i32), x: i32) {
        self.apply_tunnel(range, x, TunnelType::Vertical);
    }

    fn apply_horizontal_tunnel(&mut self, range: (i32, i32), y: i32) {
        self.apply_tunnel(range, y, TunnelType::Horizontal);
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 3 {
                self.apply_vertical_tunnel((prev.y, new.y), new.x);
                self.apply_horizontal_tunnel((prev.x, new.x), prev.y);
            } else {
                self.apply_vertical_tunnel((prev.y, new.y), prev.x);
                self.apply_horizontal_tunnel((prev.x, new.x), new.y);
            }
        }
    }
}
