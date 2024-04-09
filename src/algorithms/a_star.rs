//https://en.wikipedia.org/wiki/A*_search_algorithm

use super::Point;
use crate::algorithms::GridMap;

pub fn run(grid: &GridMap, start: &Point, end: &Point) {
    let mut open_set = vec![(start, 0)];
    let mut came_from: Vec<&Point> = Vec::new();

    let mut gscore: Vec<(Point, i32)> = Vec::new();

    for y in 0..grid.size.height {
        for x in 0..grid.size.width {
            if start.y == y && start.x == x {
                continue;
            }

            gscore.push((Point::new(x, y), i32::MAX))
        }
    }
    gscore.push((Point::new(start.x, start.y), 0));

    let mut fscore = gscore.clone();
    fscore.push((Point::new(start.x, start.y), heuristic(start, end)))
}

fn reconstruct_path(came_from: Vec<&Point>, current: (&Point, i32)) {}

//https://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html#S7
fn heuristic(node: &Point, goal: &Point) -> i32 {
    let dx = node.x - goal.x;
    let dy = node.y - goal.y;
    // 1 is the movement cost
    return 1 * (dx + dy) as i32;
}
