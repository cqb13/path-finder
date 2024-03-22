pub mod display;
pub mod tui;

use crate::display::setup::config_setup;
use crate::display::welcome::welcome;
use crate::display::Display;

pub struct Pathfinder {
    pub grid: GridMap,
    pub start: Point,
    pub end: Point,
    pub algorithm: Algorithm,
}

pub enum Algorithm {
    BreadthFirstSearch,
    DepthFirstSearch,
    Dijkstra,
    AStar,
    GreedyBestFirstSearch,
    BellmanFord,
}

pub enum GridBlock {
    Start,
    End,
    Obstacle,
    Path,
    Empty,
}

impl GridBlock {
    //▢▩▣◯□■
    pub fn to_visual_block(&self) -> &str {
        match self {
            GridBlock::Start => "▣",
            GridBlock::End => "▢",
            GridBlock::Obstacle => "■",
            GridBlock::Path => "⊡",
            GridBlock::Empty => "•",
        }
    }
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

pub struct GridElement {
    pub point: Point,
    pub grid: GridBlock,
}

impl GridElement {
    pub fn new(point: Point, grid: GridBlock) -> GridElement {
        GridElement { point, grid }
    }
}

pub struct GridSize {
    pub width: u32,
    pub height: u32,
}

impl GridSize {
    pub fn new(width: u32, height: u32) -> GridSize {
        GridSize { width, height }
    }
}

pub struct GridMap {
    pub grid: Vec<Vec<GridElement>>,
    pub size: GridSize,
}

impl GridMap {
    pub fn new(size: GridSize) -> GridMap {
        let mut grid = Vec::new();
        for _ in 0..size.height {
            let mut row = Vec::new();
            for _ in 0..size.width {
                row.push(GridElement::new(Point::new(0, 0), GridBlock::Empty));
            }
            grid.push(row);
        }
        GridMap { grid, size }
    }

    pub fn get_grid(&self, point: Point) -> &GridBlock {
        &self.grid[point.y as usize][point.x as usize].grid
    }

    pub fn set_grid(&mut self, point: Point, grid: GridBlock) {
        self.grid[point.y as usize][point.x as usize].grid = grid;
    }

    pub fn render(&self) {
        for row in &self.grid {
            for element in row {
                print!(" {} ", element.grid.to_visual_block());
            }
            println!();
        }
    }
}

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
    // first add obstacles
    // then add start and end
    
    // add some temp blocks
    grid_map.set_grid(Point::new(5, 5), GridBlock::Start);
    grid_map.set_grid(Point::new(10, 10), GridBlock::End);
    grid_map.set_grid(Point::new(7, 7), GridBlock::Obstacle);
    grid_map.set_grid(Point::new(7, 8), GridBlock::Obstacle);
    grid_map.set_grid(Point::new(7, 9), GridBlock::Obstacle);
    grid_map.set_grid(Point::new(6, 8), GridBlock::Obstacle);
    grid_map.set_grid(Point::new(8, 8), GridBlock::Path);

    grid_map.render();
}
