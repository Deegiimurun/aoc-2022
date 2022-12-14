use std::cmp::{max, Ordering};
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other { return Ordering::Equal; }

        if let (Some(left), Some(right)) = (self.is_number(), other.is_number()) {
            match left.cmp(&right) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Greater
            }
        } else if let (Some(left), Some(right)) = (self.is_list(), other.is_list()) {
            for i in 0..max(left.len(), right.len()) {
                let left = left.get(i);
                let right = right.get(i);

                if left.is_none() { return Ordering::Less; }
                if right.is_none() { return Ordering::Greater; }

                match left.unwrap().cmp(right.unwrap()) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => continue,
                    Ordering::Greater => return Ordering::Greater,
                }
            }
            return Ordering::Equal;
        } else if let (Some(left), Some(_)) = (self.is_number(), other.is_list()) {
            Packet::List(vec![Packet::Number(left)]).cmp(other)
        } else if let (Some(_), Some(right)) = (self.is_list(), other.is_number()) {
            self.cmp(&Packet::List(vec![Packet::Number(right)]))
        } else {
            panic!();
        }
    }
}


impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn parse(str: &str) -> Packet {
        let mut packets: Vec<Packet> = vec![];

        let mut temp = String::new();

        let mut chunks = str.split("").collect::<Vec<&str>>();

        chunks.remove(0);
        chunks.remove(0);
        chunks.remove(chunks.len() - 1);
        chunks.remove(chunks.len() - 1);

        let mut proper_chunks: Vec<String> = vec![];

        chunks.iter().enumerate().for_each(|(i, ch)| {
            match *ch {
                "[" | "]" | "," => {
                    proper_chunks.push(ch.to_string());
                }
                _ => {
                    if proper_chunks.is_empty() {
                        proper_chunks.push(ch.to_string());
                        return;
                    }

                    match proper_chunks.last().unwrap().as_str() {
                        "[" | "]" | "," => {
                            proper_chunks.push(ch.to_string());
                        }
                        _ => {
                            let mut temp = String::from(chunks[i - 1]);
                            temp.push_str(ch);
                            proper_chunks.remove(proper_chunks.len() - 1);
                            proper_chunks.push(temp);
                        }
                    }
                }
            }
        });

        for str in proper_chunks {
            match str.as_str() {
                "]" => {
                    temp.push_str(str.as_str());
                    if temp.matches(']').count() == temp.matches('[').count() {
                        packets.push(Packet::parse(&temp));
                        temp = String::new();
                    }
                }
                "[" => temp.push_str(str.as_str()),
                _ => {
                    if temp.is_empty() {
                        if let Ok(number) = str.parse::<i32>() {
                            packets.push(Packet::Number(number));
                        }
                    } else {
                        temp.push_str(str.as_str());
                    }
                }
            }
        }

        Packet::List(packets)
    }

    fn is_list(&self) -> Option<Vec<Packet>> {
        match self {
            Packet::Number(_) => None,
            Packet::List(v) => Some(v.clone()),
        }
    }

    fn is_number(&self) -> Option<i32> {
        match self {
            Packet::Number(n) => Some(*n),
            Packet::List(_) => None
        }
    }
}

fn main() {
    let content = fs::read_to_string("src/bin/day-13/input.txt").unwrap().replace("\r\n", "\n");

    // Part 1
    let pairs = content.split("\n\n").map(|pair| {
        let left = pair.split('\n').next().unwrap();
        let right = pair.split('\n').nth(1).unwrap();
        (Packet::parse(left), Packet::parse(right))
    }).collect::<Vec<(Packet, Packet)>>();

    println!("{}", pairs.iter().enumerate().map(|(i, pair)| {
        match pair.0 < pair.1 {
            true => {
                i + 1
            }
            false => 0,
        }
    }).sum::<usize>());

    // Part 2
    let mut packets = content.split('\n')
        .filter(|line| { !line.is_empty() })
        .map(Packet::parse).collect::<Vec<Packet>>();

    packets.push(Packet::parse("[[2]]"));
    packets.push(Packet::parse("[[6]]"));

    packets.sort();
    let mut answer = 1;

    for (i, packet) in packets.iter().enumerate() {
        if packet == &Packet::parse("[[2]]") || packet == &Packet::parse("[[6]]"){
            answer *= i + 1;
        }
    }

    println!("Part 2 answer: {answer}");
}