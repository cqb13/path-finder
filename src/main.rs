pub mod tui;
use crate::tui::option_select::OptionSelect;

fn main() {
    let options = OptionSelect::new()
        .set_title("Select an option")
        .ask();
    println!("You selected: {}", options);
}
