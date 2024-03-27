use crate::tui::number_input::NumberInput;
use crate::tui::option_select::OptionSelect;
use crate::tui::refresh_display;
use crate::{Algorithm, GridBlock, GridMap, GridSize, Point, SetupConfig};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

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
        .ask() as u16;
    let height = NumberInput::new()
        .set_message("Enter the grid height:")
        .set_min(5)
        .set_max(100)
        .ask() as u16;

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
                "Auto" => grid.generate_obstacles(),
                "Edit Auto" | "Manual" => {
                    if "Edit Auto" == obstacle_creation.as_str() {
                        grid.generate_obstacles();
                    }

                    let mut block_position = Point::new(grid.size.width / 2, grid.size.height / 2);
                    println!("Press 'S' to save and continue or 'Q' to quit");
                    loop {
                        let finished =
                            placement_loop(&mut grid, GridBlock::Obstacle, &mut block_position);
                        if finished {
                            refresh_display(grid.full_size);
                            break;
                        }
                        if grid.get_block(&block_position) == &GridBlock::Obstacle {
                            grid.set_block(&block_position, GridBlock::Empty);
                        } else {
                            grid.set_block(&block_position, GridBlock::Obstacle);
                        }
                        refresh_display(grid.full_size);
                    }
                    refresh_display(1);
                }
                _ => panic!("obstacle generation has no matching generation option"),
            };
        }
        MapBuilderMode::Start => {}
        MapBuilderMode::End => {}
    }

    grid
}

fn placement_loop(grid: &mut GridMap, block: GridBlock, block_position: &mut Point) -> bool {
    grid.render_with_selector(&block_position, &block);
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
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    println!("Quitting...");
                    std::process::exit(0);
                }
                KeyCode::Char('s') => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    return true;
                }
                KeyCode::Up => {
                    if block_position.y > 0 {
                        block_position.y -= 1;
                    } else {
                        println!("{} {}", block_position.y, grid.size.height - 1);
                        block_position.y = grid.size.height - 1;
                    }
                }
                KeyCode::Down => {
                    if block_position.y < grid.size.height - 1 {
                        block_position.y += 1;
                    } else {
                        block_position.y = 0;
                    }
                }
                KeyCode::Left => {
                    if block_position.x > 0 {
                        block_position.x -= 1;
                    } else {
                        block_position.x = grid.size.width - 1;
                    }
                }
                KeyCode::Right => {
                    if block_position.x < grid.size.width - 1 {
                        block_position.x += 1;
                    } else {
                        block_position.x = 0;
                    }
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    return false;
                }
                _ => {}
            },
            _ => {}
        }
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        refresh_display(grid.full_size);
        grid.render_with_selector(&block_position, &block);
    }
}
