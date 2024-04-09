use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

use crate::tui::refresh_display;

/**
 * Full size: the number of lines the element will take up
 * Reset size: the number of lines the user input section will take up
 */
#[derive(Debug, PartialEq, Clone)]
pub struct OptionSelect {
    title: String,
    options: Vec<String>,
    pub full_size: i32,
    reset_size: i32,
    manual_clear: bool,
}

impl OptionSelect {
    pub fn new() -> Self {
        OptionSelect {
            title: String::new(),
            options: Vec::new(),
            full_size: 1, // always a title line, the other options are added to this
            reset_size: 0,
            manual_clear: false,
        }
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn add_option(mut self, option: &str) -> Self {
        self.options.push(option.to_string());
        self.full_size += 1;
        self.reset_size += 1;
        self
    }

    pub fn manual_clear(mut self) -> Self {
        self.manual_clear = true;
        self
    }

    pub fn add_option_if_true(mut self, option: String, condition: bool) -> Self {
        if condition {
            self.options.push(option);
            self.full_size += 1;
            self.reset_size += 1;
        }
        self
    }

    pub fn ask(&self) -> String {
        let mut current_option = 0;
        if self.options.is_empty() {
            panic!("No options to select from");
        }
        println!("{}", self.title);

        loop {
            for (i, option) in self.options.iter().enumerate() {
                if i == current_option {
                    println!("> [{}] {}", i + 1, option);
                    continue;
                }
                println!("  [{}] {}", i + 1, option);
            }
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
                    KeyCode::Up => {
                        if current_option > 0 {
                            current_option -= 1;
                        } else {
                            current_option = self.options.len() - 1;
                        }
                    }
                    KeyCode::Down => {
                        if current_option < self.options.len() - 1 {
                            current_option += 1;
                        } else {
                            current_option = 0;
                        }
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        if !self.manual_clear {
                            refresh_display(self.full_size);
                        }
                        return self.options[current_option].to_string();
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
