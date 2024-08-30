use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let mut result = Self {
            left_x: 0,
            right_x: 0,
            top_y: 0,
            bottom_y: 0,
        };
        result.on_player_move(player_position);
        result
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - DISPLAY_WIDTH / 2;
        self.right_x = self.left_x + DISPLAY_WIDTH;
        self.top_y = player_position.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = self.top_y + DISPLAY_HEIGHT;
    }
}
