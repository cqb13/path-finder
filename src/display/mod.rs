pub mod setup;
pub mod welcome;

use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

pub struct Display {
    pub height: u16,
}

impl Display {
    pub fn new() -> Display {
        Display { height: 0 }
    }

    pub fn add_height(&mut self, height: u16) {
        self.height += height;
    }

    pub fn remove_height(&mut self, height: u16) {
        self.height -= height;
    }

    pub fn reset_height(&mut self) {
        self.height = 0;
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn refresh(&self) {
        for _ in 0..self.height {
            io::stdout().execute(cursor::MoveUp(1)).unwrap();
            io::stdout()
                .execute(terminal::Clear(terminal::ClearType::CurrentLine))
                .unwrap();
        }
    }
}
