use std::cmp::{max, min};
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let content = fs::read_to_string("src/bin/day-14/input.txt").unwrap().replace("\r\n", "\n");
    let mut grid = vec![vec![" "; 1000]; 1000];

    content.split('\n').for_each(|line| {
        let points = line.split(" -> ").map(|point| {
            Point {
                y: point.split(',').next().unwrap().parse::<usize>().unwrap(),
                x: point.split(',').nth(1).unwrap().parse::<usize>().unwrap(),
            }
        }).collect::<Vec<Point>>();

        for i in 0..points.len() - 1 {
            let start = points.get(i).unwrap();
            let end = points.get(i + 1);

            if end.is_none() {
                grid[start.x][start.y] = "#";
                break;
            }

            let end = end.unwrap();

            for x in min(start.x, end.x)..=max(start.x, end.x) {
                for y in min(start.y, end.y)..=max(start.y, end.y) {
                    grid[x][y] = "#";
                }
            }
        }
    });

    let mut lowest = 0;

    for (i, row) in grid.iter().enumerate().rev() {
        if row.contains(&"#") {
            lowest = i;
            break;
        }
    }


    //Part 1
    let mut grid_clone = grid.clone();
    let mut counter = 0;

    let mut sand = Point { x: 500, y: 0 };
    loop {
        if sand.y == lowest + 1 {
            break;
        }
        let top_down = Point { x: sand.x, y: sand.y + 1 };
        let left_down = Point { x: sand.x - 1, y: sand.y + 1 };
        let right_down = Point { x: sand.x + 1, y: sand.y + 1 };

        if grid_clone[top_down.y][top_down.x] == " " {
            sand = top_down;
            continue;
        }
        if grid_clone[left_down.y][left_down.x] == " " {
            sand = left_down;
            continue;
        }
        if grid_clone[right_down.y][right_down.x] == " " {
            sand = right_down;
            continue;
        }

        grid_clone[sand.y][sand.x] = ".";
        sand = Point { x: 500, y: 0 };
        counter += 1;
    }

    println!("Part 1 answer: {counter}");

    //Part 2
    let mut grid_clone = grid.clone();
    let mut counter = 0;

    grid_clone[lowest + 2] = vec!["#"; 1000];

    let mut sand = Point { x: 500, y: 0 };
    loop {
        if grid_clone[0][500] != " " {
            break;
        }

        let top_down = Point { x: sand.x, y: sand.y + 1 };
        let left_down = Point { x: sand.x - 1, y: sand.y + 1 };
        let right_down = Point { x: sand.x + 1, y: sand.y + 1 };

        if grid_clone[top_down.y][top_down.x] == " " {
            sand = top_down;
            continue;
        }
        if grid_clone[left_down.y][left_down.x] == " " {
            sand = left_down;
            continue;
        }
        if grid_clone[right_down.y][right_down.x] == " " {
            sand = right_down;
            continue;
        }

        grid_clone[sand.y][sand.x] = ".";
        sand = Point { x: 500, y: 0 };
        counter += 1;
    }

    println!("Part 2 answer: {counter}");
}