use std::fs;

struct ElfAssignment {
    min: i32,
    max: i32,
}

fn main() {
    let content = fs::read_to_string("src/bin/day-4/input.txt").unwrap().replace("\r\n","\n");

    let elf_assigment_pairs = content.split('\n').map(|line| {
        let pair_str = line.split(',').collect::<Vec<&str>>();
        let first = pair_str.first().unwrap().split('-').collect::<Vec<&str>>();
        let second = pair_str.last().unwrap().split('-').collect::<Vec<&str>>();
        (
            ElfAssignment {
                min: first.first().unwrap().parse().unwrap(),
                max: first.last().unwrap().parse().unwrap(),
            },
            ElfAssignment {
                min: second.first().unwrap().parse().unwrap(),
                max: second.last().unwrap().parse().unwrap(),
            }
        )
    }).collect::<Vec<(ElfAssignment, ElfAssignment)>>();

    //Part 1

    println!("Part 1 answer: {}", elf_assigment_pairs.iter().map(|pair| {
        (pair.0.min <= pair.1.min && pair.0.max >= pair.1.max) || (pair.0.min >= pair.1.min && pair.0.max <= pair.1.max)
    }).filter(|res| *res).count());

    //Part 2
    println!("Part 2 answer: {}", elf_assigment_pairs.iter().map(|pair| {
        (pair.0.min >= pair.1.min && pair.0.min <= pair.1.max) || (pair.0.max >= pair.1.min && pair.0.max <= pair.1.max) ||
        (pair.1.min >= pair.0.min && pair.1.min <= pair.0.max) || (pair.1.max >= pair.0.min && pair.1.max <= pair.0.max)
    }).filter(|res| *res).count());
}