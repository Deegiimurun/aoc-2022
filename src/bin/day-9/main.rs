use std::fs;

#[derive(Clone, Debug, Eq)]
struct Point {
    x: i32,
    y: i32,
    visited: Vec<(i32, i32)>,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
            visited: vec![(0, 0)],
        }
    }

    fn move_point(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
        };

        self.visited.push((self.x, self.y));
    }

    fn is_close(&self, other: &Point) -> bool {
        (-1..=1).contains(&(self.x - other.x)) && (-1..=1).contains(&(self.y - other.y))
    }

    fn chase(&mut self, other: &Point) {
        if self.is_close(other) { return; }

        if self.y != other.y {
            match self.y - other.y > 0 {
                true => self.y -= 1,
                false => self.y += 1
            }
        }

        if self.x != other.x {
            match self.x - other.x > 0 {
                true => self.x -= 1,
                false => self.x += 1
            }
        }

        if !self.visited.contains(&(self.x, self.y)) {
            self.visited.push((self.x, self.y));
        }
    }
}

#[derive(Debug)]
struct Grid {
    points: Vec<Point>,
}

impl Grid {
    fn move_head(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.step {
            self.points[0].move_point(instruction);

            for i in 1..self.points.len() {
                let prev_point = self.points[i - 1].clone();
                let point = self.points.get_mut(i).unwrap();
                point.chase(&prev_point);
            }
        };
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    step: i32,
}

impl Instruction {
    fn new(direction: &str, step: i32) -> Instruction {
        match direction.to_uppercase().as_str() {
            "L" => Instruction {
                direction: Direction::Left,
                step,
            },
            "U" => Instruction {
                direction: Direction::Up,
                step,
            },
            "R" => Instruction {
                direction: Direction::Right,
                step,
            },
            "D" => Instruction {
                direction: Direction::Down,
                step,
            },
            _ => panic!(),
        }
    }
}

fn main() {
    let content = fs::read_to_string("src/bin/day-9/input.txt").unwrap().replace("\r\n", "\n");

    let instructions: Vec<Instruction> = content.split('\n').fold(vec![], |mut acc, line| {
        let instruction: Vec<&str> = line.split(' ').collect();
        let direction = instruction[0];
        let step: i32 = instruction[1].parse().unwrap();
        acc.push(Instruction::new(direction, step));
        acc
    });

    let mut grid = Grid {
        points: vec![Point::new(); 10]
    };

    instructions.iter().for_each(|instruction| {
        grid.move_head(instruction);
    });


    //Part 1
    println!("Part 1 answer: {}", grid.points[1].visited.len());

    //Part 2
    println!("Part 2 answer: {}", grid.points[9].visited.len());
}