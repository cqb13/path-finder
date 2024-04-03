use super::{GridBlock, Point};
use crate::algorithms::GridMap;

pub fn run(mut grid: GridMap, start: &Point, end: &Point) {
    let mut visited: Vec<Vec<bool>> = Vec::new();
    let mut distance: Vec<Vec<i32>> = Vec::new();
    let mut parent: Vec<Vec<Point>> = Vec::new();
    for _ in 0..grid.size.height {
        let mut row_vis = Vec::new();
        let mut row_dis = Vec::new();
        let mut row_par = Vec::new();
        for _ in 0..grid.size.width {
            row_vis.push(false);
            row_dis.push(i32::MAX);
            row_par.push(Point::new(0, 0));
        }
        visited.push(row_vis);
        distance.push(row_dis);
        parent.push(row_par);
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
    println!("Path Length: {}", distance[end.y as usize][end.x as usize]);
}
