use std::fs;

#[derive(Debug)]
struct Rucksack {
    full: String,
    first_half: String,
    second_half: String,
    item_type: char,
    priority: i32,
}

fn main() {
    let contents = fs::read_to_string("src/bin/day-3/input.txt").unwrap();

    let rucksacks: Vec<Rucksack> = contents.split('\n').map(|line| {
        let first_half = line.to_string()[0..line.len() / 2].to_string();
        let second_half = line.to_string()[line.len() / 2..].to_string();
        let mut item_type: char = '\0';
        let mut priorty = 0;

        for x in first_half.chars() {
            if second_half.contains(x) {
                item_type = x;
                break;
            }
        }

        priorty = if item_type.is_lowercase() {
            item_type as i32 - 96
        } else {
            item_type as i32 - 38
        };

        Rucksack {
            full: line.to_string(),
            first_half,
            second_half,
            item_type,
            priority: priorty,
        }
    }).collect();

    //Part 1
    println!("Part 1 answer: {}", rucksacks.iter().map(|rucksack| rucksack.priority).sum::<i32>());

    //Part 2
    println!("Part 2 answer: {}", rucksacks.chunks(3).map(|chunk| {
        let first = chunk.get(0).unwrap();
        let second = chunk.get(1).unwrap();
        let third = chunk.get(2).unwrap();

        for x in first.full.chars() {
            if second.full.contains(x) && third.full.contains(x) {
                return if x.is_lowercase() {
                    x as i32 - 96
                } else {
                    x as i32 - 38
                }
            }
        }

        0
    }).sum::<i32>())
}