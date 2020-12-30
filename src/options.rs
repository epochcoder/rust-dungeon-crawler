use crate::prelude::*;

const DEFAULT_NUM_ROOMS: usize = 10;
const DEFAULT_ROOM_SIZE: i32 = 10;

#[derive(Clone)]
pub enum GameMode {
    Play, Menu, Quit, Restart
}

#[derive(Clone)]
pub struct GameOptions {
    pub max_rooms: usize,
    pub room_size: i32,
    pub mode: GameMode,
}

impl GameOptions {
    pub fn new() -> Self {
        Self {
            max_rooms: DEFAULT_NUM_ROOMS,
            room_size: DEFAULT_ROOM_SIZE,
            mode: GameMode::Play
        }
    }

    pub fn handle_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::LBracket if self.room_size > 4 => self.room_size = self.room_size - 1,
                VirtualKeyCode::RBracket => self.room_size = self.room_size + 1,
                VirtualKeyCode::Comma if self.max_rooms > 1 => self.max_rooms = self.max_rooms - 1,
                VirtualKeyCode::Period => self.max_rooms = self.max_rooms + 1,
                _ => {}
            }
        }
    }
}
