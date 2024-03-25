use crate::tui::number_input::NumberInput;
use crate::tui::option_select::OptionSelect;
use crate::{Algorithm, GridMap, GridSize, SetupConfig};

pub enum MapBuilderMode {
    Obstacle,
    Start,
    End,
}

pub fn config_setup() -> SetupConfig {
    let width = NumberInput::new()
        .set_message("Enter the gird width:")
        .set_min(5)
        .set_max(100)
        .ask() as u32;
    let height = NumberInput::new()
        .set_message("Enter the grid height:")
        .set_min(5)
        .set_max(100)
        .ask() as u32;

    let algorithm_selection = OptionSelect::new()
        .set_title("Select an algorithm:")
        .add_option("Breadth First Search")
        .add_option("Depth First Search")
        .add_option("Dijkstra")
        .add_option("A Star")
        .add_option("Greedy Best First Search")
        .add_option("Bellman Ford")
        .ask();
    let algorithm = match algorithm_selection.as_str() {
        "Breadth First Search" => Algorithm::BreadthFirstSearch,
        "Depth First Search" => Algorithm::DepthFirstSearch,
        "Dijkstra" => Algorithm::Dijkstra,
        "A Star" => Algorithm::AStar,
        "Greedy Best First Search" => Algorithm::GreedyBestFirstSearch,
        "Bellman Ford" => Algorithm::BellmanFord,
        _ => panic!("algorithm selection has no matching algorithm"),
    };

    SetupConfig::new(GridSize::new(width, height), algorithm)
}

pub fn map_builder(mode: MapBuilderMode, mut grid: GridMap) -> GridMap {
    match mode {
        MapBuilderMode::Obstacle => {
            let obstacle_creation = OptionSelect::new()
                .set_title("Select obstacle generation method:")
                .add_option("Manual")
                .add_option("Auto")
                .add_option("Edit Auto")
                .ask();

            match obstacle_creation.as_str() {
                "Manual" => {}
                "Auto" => grid.generate_obstacles(),
                "Edit Auto" => {
                    grid.generate_obstacles();
                }
                _ => panic!("obstacle generation has no matching generation option"),
            };
        }
        MapBuilderMode::Start => {}
        MapBuilderMode::End => {}
    }

    grid
}
