use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

use crate::tui::refresh_display;

/**
 * Full size: the number of lines the element will take up
 */
pub struct Confirm {
    message: String,
    pub full_size: u16,
}

impl Confirm {
    pub fn new() -> Self {
        Confirm {
            message: String::new(),
            full_size: 2,
        }
    }

    pub fn set_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn ask(&self) -> bool {
        println!("{}", self.message);
        println!("Press enter to confirm or c to cancel");

        loop {
            terminal::enable_raw_mode().expect("Failed to enable raw mode");

            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Char('c') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        refresh_display(self.full_size);
                        return false;
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        refresh_display(self.full_size);
                        return true;
                    }
                    _ => {}
                },
                _ => {}
            }
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
        }
    }
}
