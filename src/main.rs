pub mod tui;
pub mod display;

use crate::display::welcome::welcome;
use crate::display::setup::config_setup;
use crate::display::Display;

pub struct Pathfinding {
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
    Empty,
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
}

pub struct SetupConfig {
    grid_size: GridSize,
    algorithm: Algorithm,
}

impl SetupConfig {
    pub fn new(grid_size: GridSize, algorithm: Algorithm) -> SetupConfig {
        SetupConfig { grid_size, algorithm }
    }
}

fn main() {
    welcome();
    let base_config = config_setup();


}
