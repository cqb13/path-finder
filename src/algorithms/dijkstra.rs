use super::{GridBlock, Point};
use crate::algorithms::GridMap;

pub fn run(mut grid: GridMap, start: &Point, end: &Point) {
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for _ in 0..grid.size.height {
        let mut row = Vec::new();
        for _ in 0..grid.size.width {
            row.push(false);
        }
        visited.push(row);
    }

    let mut distance: Vec<Vec<i32>> = Vec::new();
    for _ in 0..grid.size.height {
        let mut row = Vec::new();
        for _ in 0..grid.size.width {
            row.push(i32::MAX);
        }
        distance.push(row);
    }

    let mut parent: Vec<Vec<Point>> = Vec::new();
    for _ in 0..grid.size.height {
        let mut row = Vec::new();
        for _ in 0..grid.size.width {
            row.push(Point::new(0, 0));
        }
        parent.push(row);
    }

    let mut queue: Vec<Point> = Vec::new();
    queue.push(start.clone());
    visited[start.y as usize][start.x as usize] = true;
    distance[start.y as usize][start.x as usize] = 0;

    while !queue.is_empty() {
        let current = queue.remove(0);
        if current.x == end.x && current.y == end.y {
            break;
        }

        let surrounding_blocks = grid.get_surrounding_blocks(&current);
        for (block, point) in surrounding_blocks {
            if block == GridBlock::Obstacle {
                continue;
            }
            if !visited[point.y as usize][point.x as usize] {
                visited[point.y as usize][point.x as usize] = true;
                distance[point.y as usize][point.x as usize] =
                    distance[current.y as usize][current.x as usize] + 1;
                parent[point.y as usize][point.x as usize] = current;
                queue.push(point);
            }
        }
    }

    let mut current = end.clone();
    while current.x != start.x || current.y != start.y {
        grid.set_block(&current, &GridBlock::Path);
        current = parent[current.y as usize][current.x as usize];
    }

    grid.set_block(&start, &GridBlock::Start);
    grid.set_block(&end, &GridBlock::End);
    grid.render();
}
