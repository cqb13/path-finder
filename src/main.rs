pub mod algorithms;
pub mod display;
pub mod tui;

use crate::algorithms::{Algorithm, GridBlock, GridMap, GridSize, Pathfinder, Point};
use crate::display::setup::{config_setup, map_builder, MapBuilderMode};
use crate::display::welcome::welcome;
use crate::tui::confirm::Confirm;
use crate::tui::refresh_display;

pub struct SetupConfig {
    grid_size: GridSize,
    algorithm: Algorithm,
}

impl SetupConfig {
    pub fn new(grid_size: GridSize, algorithm: Algorithm) -> SetupConfig {
        SetupConfig {
            grid_size,
            algorithm,
        }
    }
}

fn main() {
    welcome();
    let base_config = config_setup();
    let mut grid_map = GridMap::new(base_config.grid_size);
    grid_map = map_builder(MapBuilderMode::Obstacle, grid_map);
    grid_map = map_builder(MapBuilderMode::Start, grid_map);
    grid_map = map_builder(MapBuilderMode::End, grid_map);
    grid_map.render();

    let confirm_start = Confirm::new()
        .set_message("Would you like to start the algorithm?")
        .ask();

    if !confirm_start {
        return;
    }

    refresh_display(grid_map.full_size);

    let pathfinder = Pathfinder {
        start: grid_map.start,
        end: grid_map.end,
        grid: grid_map,
        algorithm: base_config.algorithm,
    };

    pathfinder.run();
}
