use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

use crate::tui::refresh_display;

/**
 * Full size: the number of lines the element will take up
 * Reset size: the number of lines the user input section will take up
 */
pub struct NumberInput {
    message: String,
    min: i32,
    max: Option<i32>,
    pub full_size: u16,
    reset_size: u16,
    manual_clear: bool,
}

impl NumberInput {
    pub fn new() -> Self {
        NumberInput {
            message: String::new(),
            min: 0,
            max: None,
            full_size: 3,
            reset_size: 1,
            manual_clear: false,
        }
    }

    pub fn set_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn set_min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    pub fn set_max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    pub fn manual_clear(mut self) -> Self {
        self.manual_clear = true;
        self
    }

    pub fn ask(&self) -> i32 {
        println!("{}", self.message);

        if self.max.is_none() {
            println!("Number must be at least {}", self.min);
        } else {
            println!(
                "Number must be between {} and {}",
                self.min,
                self.max.unwrap()
            );
        }

        let mut current_number_string = String::new();

        loop {
            println!("> {}", current_number_string);
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
                    KeyCode::Char(c) => {
                        if c.is_numeric() {
                            current_number_string.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        current_number_string.pop();
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        let current_number = current_number_string.parse::<i32>().unwrap_or(0);
                        if current_number >= self.min {
                            if let Some(max) = self.max {
                                if current_number <= max {
                                    if !self.manual_clear {
                                        refresh_display(self.full_size);
                                    }
                                    return current_number;
                                }
                            } else {
                                if !self.manual_clear {
                                    refresh_display(self.full_size);
                                }
                                return current_number;
                            }
                        }
                        current_number_string = String::new();
                    }
                    _ => {}
                },
                _ => {}
            }

            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            refresh_display(self.reset_size);
        }
    }
}
