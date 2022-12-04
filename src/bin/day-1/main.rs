use std::{fs};


fn main() {
    let contents = fs::read_to_string("src/bin/day-1/input.txt").unwrap();
    let mut net_calories: Vec<i32> = Vec::new();

    for calories in contents.split("\n\n") {
        let inventory: Vec<i32> = calories
            .split('\n')
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        net_calories.push(inventory.iter().sum());
    }

    net_calories.sort();
    net_calories.reverse();

    //Part 1
    println!("Part 1 answer: {:?}", net_calories.first().unwrap());
    //Part 2
    println!("Part 2 answer: {:?}", net_calories[0..3].iter().sum::<i32>());
}
