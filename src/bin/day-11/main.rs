use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: (String, String),
    test: (usize, usize, usize),
    inspection_count: usize,
}

fn main() {
    let content = fs::read_to_string("src/bin/day-11/input.txt").unwrap().replace("\r\n", "\n");
    let mut monkeys: Vec<Monkey> = vec![];

    content.split("\n\n").for_each(|instruction| {
        let instruction: Vec<&str> = instruction.split('\n').skip(1).map(|l| l.trim()).collect();
        let starting_items = instruction[0].split(": ").nth(1).unwrap().split(", ").map(|item| item.parse().unwrap()).collect::<Vec<usize>>();
        let operation_vec = instruction[1].split(" = old ").nth(1).unwrap().split(' ').collect::<Vec<&str>>();
        let disible: usize = instruction[2].split(' ').last().unwrap().parse().unwrap();
        let idx_true: usize = instruction[3].split(' ').last().unwrap().parse().unwrap();
        let idx_false: usize = instruction[4].split(' ').last().unwrap().parse().unwrap();

        let operation_operator = operation_vec[0].to_string();
        let operation_value = operation_vec[1].to_string();

        let monkey = Monkey {
            items: starting_items,
            operation: (operation_operator, operation_value),
            test: (disible, idx_true, idx_false),
            inspection_count: 0,
        };

        monkeys.push(monkey);
    });

    //Part 1
    let mut p1_monkeys = monkeys.clone();
    for _ in 0..20 {
        for i in 0..p1_monkeys.len() {
            for j in 0..p1_monkeys[i].items.len() {
                let monkey = p1_monkeys[i].clone();
                let worry_level = monkey.items[j];

                let operation_value: usize = match monkey.operation.1.as_str() {
                    "old" => worry_level,
                    other => other.parse().unwrap(),
                };

                let operation_result = match monkey.operation.0.as_str() {
                    "*" => (worry_level * operation_value) / 3,
                    "+" => (worry_level + operation_value) / 3,
                    _ => panic!(),
                };

                if operation_result % monkey.test.0 == 0 {
                    p1_monkeys[monkey.test.1].items.push(operation_result);
                } else {
                    p1_monkeys[monkey.test.2].items.push(operation_result);
                }
            }
            p1_monkeys[i].inspection_count += p1_monkeys[i].items.len() as usize;
            p1_monkeys[i].items = vec![];
        }
    };
    let mut x = p1_monkeys.iter().map(|monkey| monkey.inspection_count).collect::<Vec<usize>>();
    x.sort();
    x.reverse();

    println!("Part 1 answer: {}", x[0] * x[1]);

    //Part 2
    let mut divisor = 1;
    for m in &monkeys {
        divisor *= m.test.0;
    }

    let mut p2_monkeys = monkeys;
    for _ in 0..10000 {
        for i in 0..p2_monkeys.len() {
            for j in 0..p2_monkeys[i].items.len() {
                let monkey = p2_monkeys[i].clone();
                let worry_level = monkey.items[j];

                let operation_value: usize = match monkey.operation.1.as_str() {
                    "old" => worry_level,
                    other => other.parse().unwrap(),
                };

                let operation_result = match monkey.operation.0.as_str() {
                    "*" => worry_level * operation_value,
                    "+" => worry_level + operation_value,
                    _ => panic!(),
                };

                if operation_result % monkey.test.0 == 0 {
                    p2_monkeys[monkey.test.1].items.push(operation_result % divisor);
                } else {
                    p2_monkeys[monkey.test.2].items.push(operation_result % divisor);
                }
            }
            p2_monkeys[i].inspection_count += p2_monkeys[i].items.len() as usize;
            p2_monkeys[i].items = vec![];
        }
    };
    let mut x = p2_monkeys.iter().map(|monkey| monkey.inspection_count).collect::<Vec<usize>>();
    x.sort();
    x.reverse();

    println!("Part 2 answer: {}", x[0] * x[1]);
}