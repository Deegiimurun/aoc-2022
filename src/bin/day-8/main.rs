use std::fs;

type Grid<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Tree {
    left_visible: bool,
    top_visible: bool,
    right_visible: bool,
    bottom_visible: bool,
    scenic_score: i32,
}

impl Tree {
    fn visible_from_outside(&self) -> bool {
        self.left_visible || self.top_visible || self.right_visible || self.bottom_visible
    }
}

fn main() {
    let content = fs::read_to_string("src/bin/day-8/input.txt").unwrap().replace("\r\n", "\n");

    let mut tree_grid: Grid<Tree> = vec![];
    let height_grid: Grid<i32> = content
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|ch|
                    ch.to_digit(10)
                        .unwrap() as i32)
                .collect()
        })
        .collect();

    for i in 0..height_grid.len() {
        let mut tree_row: Vec<Tree> = vec![];
        for j in 0..height_grid[i].len() {
            let height = height_grid[i][j];

            let mut tree = Tree {
                left_visible: true,
                top_visible: true,
                right_visible: true,
                bottom_visible: true,
                scenic_score: 1,
            };

            let mut visible = 0;
            for x in (0..j).rev() {
                visible += 1;
                if height <= height_grid[i][x] {
                    tree.left_visible = false;
                    break;
                }
            }
            tree.scenic_score *= visible;

            let mut visible = 0;
            for x in j + 1..height_grid[i].len() {
                visible += 1;
                if height <= height_grid[i][x] {
                    tree.right_visible = false;
                    break;
                }
            }
            tree.scenic_score *= visible;

            let mut visible = 0;
            for y in (0..i).rev() {
                visible += 1;
                if height <= height_grid[y][j] {
                    tree.top_visible = false;
                    break;
                }
            }
            tree.scenic_score *= visible;

            let mut visible = 0;
            for y in i + 1..height_grid.len() {
                visible += 1;
                if height <= height_grid[y][j] {
                    tree.bottom_visible = false;
                    break;
                }
            }
            tree.scenic_score *= visible;

            tree_row.push(tree);
        }
        tree_grid.push(tree_row);
    }

    //Part 1
    println!("Part 1 answer: {}", tree_grid.iter().map(|row| row.iter().map(|tree|
        match tree.visible_from_outside() {
            true => 1,
            false => 0,
        }
    ).sum::<i32>()).sum::<i32>());

    //Part 2
    println!("Part 2 answer: {}", tree_grid.iter().map(|row| row.iter().map(|tree| tree.scenic_score).max().unwrap()).max().unwrap());
}