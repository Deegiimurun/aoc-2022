use std::fs;

type Crate = Vec<char>;
type Instruction = (i32, i32, i32);

fn main() {
    let content = fs::read_to_string("src/bin/day-5/input.txt").unwrap().replace("\r\n","\n");

    let instructions = content
        .split("\n\n")
        .collect::<Vec<&str>>()
        .last().unwrap()
        .split('\n')
        .map(|line| {
            let instruction_vec = line
                .replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
                .split(' ').map(|ch| ch.parse().unwrap()).collect::<Vec<i32>>();

            (*instruction_vec.first().unwrap(),
             *instruction_vec.get(1).unwrap(),
             *instruction_vec.get(2).unwrap())
        })
        .collect::<Vec<Instruction>>();
    let crates_str = *content.split("\n\n").collect::<Vec<&str>>().first().unwrap();

    let mut crates_rev = crates_str
        .split('\n')
        .map(|line|
            line.chars()
                .skip(1)
                .step_by(4)
                .collect::<Crate>()
        ).collect::<Vec<Crate>>();

    let max = crates_rev.iter().map(|line| line.len()).max().unwrap();
    crates_rev.remove(crates_rev.len() - 1);

    let mut crates: Vec<Crate> = Vec::new();

    for i in 0..max {
        let mut new_crate: Crate = Vec::new();

        crates_rev.iter().rev().for_each(|line| {
            match line.get(i) {
                None => {}
                Some(ch) => {
                    if *ch != ' ' {
                        new_crate.push(*ch);
                    }
                }
            };
        });

        crates.push(new_crate);
    }


    //Part 1
    let mut crates_clone = crates.clone();

    instructions.iter().for_each(|instruction| {
        for _ in 0..instruction.0 {
            let popped_char = crates_clone.get_mut((instruction.1 - 1) as usize).unwrap().pop().unwrap();
            crates_clone.get_mut((instruction.2 - 1) as usize).unwrap().push(popped_char);
        }
    });

    println!("Part 1 answer: {}", crates_clone.iter_mut().map(|c| c.pop().unwrap()).collect::<String>());

    //Part 2
    let mut crates_clone = crates.clone();

    instructions.iter().for_each(|instruction| {
        let mut popped_chars: Vec<char> = vec![];
        for _ in 0..instruction.0 {
            popped_chars.push(crates_clone.get_mut((instruction.1 - 1) as usize).unwrap().pop().unwrap());
        }
        popped_chars.reverse();
        crates_clone.get_mut((instruction.2 - 1) as usize).unwrap().append(&mut popped_chars);
    });

    println!("Part 2 answer: {}", crates_clone.iter_mut().map(|c| c.pop().unwrap()).collect::<String>());
}