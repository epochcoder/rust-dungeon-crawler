use crate::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let half_width = DISPLAY_WIDTH / 2;
        let half_height = DISPLAY_HEIGHT / 2;

        Self {
            left_x: player_position.x - half_width,
            right_x: player_position.x + half_width,
            top_y: player_position.y - half_height,
            bottom_y: player_position.y + half_height,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        let half_width = DISPLAY_WIDTH / 2;
        let half_height = DISPLAY_HEIGHT / 2;

        self.left_x = player_position.x - half_width;
        self.right_x = player_position.x + half_width;
        self.top_y = player_position.y - half_height;
        self.bottom_y = player_position.y + half_height;
    }
}
