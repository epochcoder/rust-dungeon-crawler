use crate::prelude::*;

/// A resource that provides a limited view around a certain point
/// If bounds are exceeded, they are constrained
#[derive(Debug)]
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            left_x: 0,
            right_x: 0,
            top_y: 0,
            bottom_y: 0
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        let half_width = DISPLAY_WIDTH / 2;
        let half_height = DISPLAY_HEIGHT / 2;

        self.left_x = player_position.x - half_width;
        self.right_x = player_position.x + half_width;
        self.top_y = player_position.y - half_height;
        self.bottom_y = player_position.y + half_height;

        if self.bottom_y >= SCREEN_HEIGHT {
            self.bottom_y = SCREEN_HEIGHT;
            self.top_y = DISPLAY_HEIGHT;
        }

        if self.right_x >= SCREEN_WIDTH {
            self.right_x = SCREEN_WIDTH;
            self.left_x = DISPLAY_WIDTH;
        }

        if self.left_x < 0 {
            self.left_x = 0;
            self.right_x = DISPLAY_WIDTH;
        }

        if self.top_y < 0 {
            self.top_y = 0;
            self.bottom_y = DISPLAY_HEIGHT;
        }
    }
}
