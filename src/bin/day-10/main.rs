use std::fs;

fn main() {
    let content = fs::read_to_string("src/bin/day-10/input.txt").unwrap().replace("\r\n", "\n");

    //Part 1
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut sum = 0;

    content.split("\n").for_each(|line| {
        let mut add_to_sum = |cycle: i32, x: i32|  {
            if (cycle -20) % 40 == 0 || cycle == 20 {
                sum += x * cycle;
            }
        };

        if line == "noop" {
            cycle += 1;
            add_to_sum(cycle, x);
        } else {
            cycle += 1;
            add_to_sum(cycle, x);
            cycle += 1;
            add_to_sum(cycle, x);
            x += line.split(' ').last().unwrap().parse::<i32>().unwrap();
        }
    });

    println!("Part 1 answer: {}", sum);

    //Part2
    println!("Part 2 answer: ");

    let mut cycle = 0;
    let mut x: i32 = 1;

    content.split("\n").for_each(|line| {
        let mut draw_row = |cycle: i32, x: i32|  {
            let current_crt = (cycle - 1) % 40;
            if current_crt == x || current_crt == x - 1 || current_crt == x + 1 {
                print!("#");
            } else {
                print!(" ");
            }
            if cycle % 40  == 0 {
                println!();
            }
        };

        if line == "noop" {
            cycle += 1;
            draw_row(cycle, x);
        } else {
            cycle += 1;
            draw_row(cycle, x);
            cycle += 1;
            draw_row(cycle, x);
            x += line.split(' ').last().unwrap().parse::<i32>().unwrap();
        }
    })
}