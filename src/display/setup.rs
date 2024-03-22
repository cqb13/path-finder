use crate::display::Display;
use crate::tui::number_input::NumberInput;
use crate::tui::option_select::OptionSelect;
use crate::{Algorithm, GridMap, GridSize, SetupConfig};

pub enum MapBuilderMode {
    Obstacle,
    Start,
    End,
}

enum ObstacleGeneration {
    Random,
    Manual,
}

pub fn config_setup() -> SetupConfig {
    let mut display = Display::new();
    let width_selector = NumberInput::new()
        .set_message("Enter the gird width:")
        .set_min(5)
        .set_max(100);
    display.add_height(width_selector.full_size);
    let width = width_selector.ask() as u32;
    display.refresh();
    let height = NumberInput::new()
        .set_message("Enter the grid height:")
        .set_min(5)
        .set_max(100)
        .ask() as u32;
    display.refresh();
    display.reset_height();

    let algorithm_selector = OptionSelect::new()
        .set_title("Select an algorithm:")
        .add_option("Breadth First Search")
        .add_option("Depth First Search")
        .add_option("Dijkstra")
        .add_option("A Star")
        .add_option("Greedy Best First Search")
        .add_option("Bellman Ford");
    display.add_height(algorithm_selector.full_size);
    let algorithm_selection = algorithm_selector.ask();

    let algorithm = match algorithm_selection.as_str() {
        "Breadth First Search" => Algorithm::BreadthFirstSearch,
        "Depth First Search" => Algorithm::DepthFirstSearch,
        "Dijkstra" => Algorithm::Dijkstra,
        "A Star" => Algorithm::AStar,
        "Greedy Best First Search" => Algorithm::GreedyBestFirstSearch,
        "Bellman Ford" => Algorithm::BellmanFord,
        _ => panic!("algorithm selection has no matching algorithm"),
    };

    display.refresh();

    SetupConfig::new(GridSize::new(width, height), algorithm)
}

pub fn map_builder(mode: MapBuilderMode, grid: &mut GridMap) -> GridMap {
    unimplemented!()
}
