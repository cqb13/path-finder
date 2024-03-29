pub mod a_star;
pub mod bellman_ford;
pub mod breadth_first_search;
pub mod depth_first_search;
pub mod dijkstra;
pub mod greedy_best_first_search;

use crate::display::setup::MapCrowding;
use rand::Rng;

pub struct Pathfinder {
    pub grid: GridMap,
    pub start: Point,
    pub end: Point,
    pub algorithm: Algorithm,
}

pub trait PathFindingAlgorithm {
    fn get_surrounding_blocks(&self, grid: &GridMap, point: &Point) {}
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

impl PartialEq for GridBlock {
    fn eq(&self, other: &Self) -> bool {
        match self {
            GridBlock::Start => match other {
                GridBlock::Start => true,
                _ => false,
            },
            GridBlock::End => match other {
                GridBlock::End => true,
                _ => false,
            },
            GridBlock::Obstacle => match other {
                GridBlock::Obstacle => true,
                _ => false,
            },
            GridBlock::Path => match other {
                GridBlock::Path => true,
                _ => false,
            },
            GridBlock::Empty => match other {
                GridBlock::Empty => true,
                _ => false,
            },
        }
    }
}

impl GridBlock {
    pub fn to_visual_block(&self) -> &str {
        match self {
            GridBlock::Start => "▣",
            GridBlock::End => "▢",
            GridBlock::Obstacle => "■",
            GridBlock::Path => "⊡",
            GridBlock::Empty => "•",
        }
    }

    pub fn to_name(&self) -> &str {
        match self {
            GridBlock::Start => "Start",
            GridBlock::End => "End",
            GridBlock::Obstacle => "Obstacle",
            GridBlock::Path => "Path",
            GridBlock::Empty => "Empty",
        }
    }

    pub fn to_block(&self) -> GridBlock {
        match self {
            GridBlock::Start => GridBlock::Start,
            GridBlock::End => GridBlock::End,
            GridBlock::Obstacle => GridBlock::Obstacle,
            GridBlock::Path => GridBlock::Path,
            GridBlock::Empty => GridBlock::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
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
    pub width: u16,
    pub height: u16,
}

impl GridSize {
    pub fn new(width: u16, height: u16) -> GridSize {
        GridSize { width, height }
    }
}

pub struct GridMap {
    pub grid: Vec<Vec<GridElement>>,
    pub size: GridSize,
    pub full_size: u16,
    pub start: Point,
    pub end: Point,
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
        let full_size = size.height;
        GridMap {
            grid,
            size,
            full_size,
            start: Point::new(0, 0),
            end: Point::new(0, 0),
        }
    }

    pub fn get_block(&self, point: &Point) -> &GridBlock {
        &self.grid[point.y as usize][point.x as usize].grid
    }

    pub fn set_block(&mut self, point: &Point, block: &GridBlock) {
        self.grid[point.y as usize][point.x as usize].grid = block.to_block();
    }

    pub fn set_start(&mut self, point: &Point) {
        self.start = Point::new(point.x, point.y);
    }

    pub fn set_end(&mut self, point: &Point) {
        self.end = Point::new(point.x, point.y);
    }

    /**
     * Populates the current object's grid with obstacles in a semi-random fashion. This method performs two primary actions:
     *
     * 1. Direct Obstacle Placement:
     *    - For a predetermined number of times (`n`), it attempts to place rectangular obstacles within the grid.
     *    - Each obstacle's dimensions are determined by a base minimum size plus a random additional size, ensuring variability.
     *    - The placement is tried multiple times (`retries`) to find a suitable location within the grid where the obstacle can fit.
     *    - The inner area of each rectangular obstacle (excluding the edges) is marked as `Obstacle` on the grid.
     *
     * 2. Random Obstacle Sprinkling:
     *    - After the direct placement, the method iterates through the entire grid.
     *    - For each cell not already marked as an obstacle, there's a fixed chance based on map crowding that it will be marked as an obstacle.
     *
     * This approach ensures a mix of sizable, strategically placed obstacles and smaller, randomly distributed ones, enhancing the grid's complexity.
     *
     * Preconditions:
     * - The grid (`self.grid`), its dimensions (`self.size.width` and `self.size.height`), and the obstacle enum (`GridBlock::Obstacle`) are defined.
     * - The `rand::thread_rng().gen_range()` function is used for random number generation.
     *
     * Postconditions:
     * - The grid will contain a mix of rectangular obstacles and randomly placed obstacles, subject to the limits of `n` obstacles and the `p` percent chance for any unmarked cell.
     */
    pub fn generate_obstacles(&mut self, crowding: MapCrowding) {
        let min = crowding.min_obstacle_size();
        let max = crowding.max_obstacle_size();
        let diff = max - min;
        let n = 5;
        let retries = 10;

        for _i1 in 0..n {
            let mut x1: u16;
            let mut y1: u16;
            let mut x2: u16;
            let mut y2: u16;
            for _ in 0..retries {
                x1 = rand::thread_rng().gen_range(0..self.size.width);
                y1 = rand::thread_rng().gen_range(0..self.size.height);

                x2 = x1 + min + rand::thread_rng().gen_range(0..diff);
                y2 = y1 + min + rand::thread_rng().gen_range(0..diff);

                if x2 < self.size.width && y2 < self.size.height {
                    let mut tmp = Vec::new();
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            let index = x + y * self.size.width;
                            if self.grid[y as usize][x as usize].grid == GridBlock::Obstacle {
                                continue;
                            }
                            if x > x1 && x < x2 && y > y1 && y < y2 {
                                tmp.push(index);
                            }
                        }
                    }
                    for index in tmp {
                        self.grid[index as usize / self.size.width as usize]
                            [index as usize % self.size.width as usize]
                            .grid = GridBlock::Obstacle;
                    }
                    break;
                }
            }
        }

        for i in 0..self.size.width {
            for j in 0..self.size.height {
                let _index = i + j * self.size.width;
                if self.grid[j as usize][i as usize].grid == GridBlock::Obstacle {
                    continue;
                }
                if rand::thread_rng().gen_range(0.0..1.0) < crowding.convert_chance() {
                    self.grid[j as usize][i as usize].grid = GridBlock::Obstacle;
                }
            }
        }
    }

    pub fn render_with_selector(&self, point: &Point, selector: &GridBlock) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                if point.x == x as u16 && point.y == y as u16 {
                    print!(" {} ", selector.to_visual_block());
                } else {
                    print!(" {} ", element.grid.to_visual_block());
                }
            }
            println!();
        }
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
