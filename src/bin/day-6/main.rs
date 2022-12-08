use std::fs;

fn main() {
    let content = fs::read_to_string("src/bin/day-6/input.txt").unwrap();

    //Part 1
    let mut buffer = content[0..3].to_string();

    'outer: for i in 3..content.len() {
        buffer.push(content.chars().nth(i).unwrap());
        let chars = &content[i-3..=i];

        for char in chars.chars() {
            if chars.chars().filter(|ch| ch == &char).count() > 1 {
                continue 'outer;
            }
        }

        break;
    }

    println!("Part 1 answer: {}", buffer.len());

    //Part 2
    let mut buffer = content[0..13].to_string();

    'outer: for i in 13..content.len() {
        buffer.push(content.chars().nth(i).unwrap());
        let chars = &content[i-13..=i];

        for char in chars.chars() {
            if chars.chars().filter(|ch| ch == &char).count() > 1 {
                continue 'outer;
            }
        }

        break;
    }

    println!("Part 1 answer: {}", buffer.len());
}