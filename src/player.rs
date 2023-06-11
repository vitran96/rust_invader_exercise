use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    // Spawn
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2, // Middle width
            y: NUM_ROWS - 1, // Bottom
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    // Drawing Player
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
    }
}
