//https://en.wikipedia.org/wiki/A*_search_algorithm

use super::{GridBlock, Point};
use crate::algorithms::GridMap;

pub fn run(grid: &GridMap, start: &Point, end: &Point) {
    let mut open_set = vec![start];
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

    let mut fscore = Vec::new();
    fscore.push((Point::new(start.x, start.y), heuristic(start, end)));
    fscore.append(&mut gscore.clone());
    let temp_gscore = gscore.clone();
    while !open_set.is_empty() {
        let current = calculate_current_node(&open_set, &fscore);
        if current.0 == *end {
            println!("Found the goal")
        }

        let neighbors = grid.get_surrounding_blocks(&current.0);
        for neighbor in neighbors {
            if neighbor.0 == GridBlock::Obstacle {
                continue;
            }

            let gscore_of_neighbor: Option<&(Point, i32)> =
                temp_gscore.iter().find(|x| x.0 == neighbor.1);
            let gscore_of_current = temp_gscore.iter().find(|x| x.0 == current.0);
            if gscore_of_neighbor.is_none() || gscore_of_current.is_none() {
                panic!("Failed to find gscore of current of neighbor");
            }
            let tentative_gscore = gscore_of_current.unwrap().1 + 1;

            if tentative_gscore < gscore_of_neighbor.unwrap().1 {
                came_from.push(&gscore_of_current.unwrap().0);

                gscore
                    .iter_mut()
                    .find(|x| x.0 == neighbor.1)
                    .map(|x| x.1 = tentative_gscore);
                fscore.iter_mut().find(|x| x.0 == neighbor.1).unwrap().1 =
                    tentative_gscore + heuristic(&neighbor.1, end);

                if !open_set.contains(&&neighbor.1) {
                    open_set.push(&gscore_of_neighbor.unwrap().0);
                }
            }
        }
    }
}

fn reconstruct_path(came_from: Vec<&Point>, current: (&Point, i32)) {}

// node in open_set with lowest fscore value
fn calculate_current_node(open_set: &Vec<&Point>, fscore: &Vec<(Point, i32)>) -> (Point, i32) {
    let mut current = open_set[0];
    let mut current_fscore = fscore[0].1;
    for (i, node) in open_set.iter().enumerate() {
        if fscore[i].1 < current_fscore {
            current = node;
            current_fscore = fscore[i].1;
        }
    }

    return (current.clone(), current_fscore);
}

//https://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html#S7
fn heuristic(node: &Point, goal: &Point) -> i32 {
    let dx = node.x - goal.x;
    let dy = node.y - goal.y;
    let cost = 1 * (dx + dy);
    cost
}
