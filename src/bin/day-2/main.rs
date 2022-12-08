extern crate core;

use std::cmp::Ordering;
use std::fs;

#[derive(Copy, Debug, Eq, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Hand::Rock => {
                match other {
                    Hand::Rock => Ordering::Equal,
                    Hand::Paper => Ordering::Less,
                    Hand::Scissor => Ordering::Greater,
                }
            }
            Hand::Paper => {
                match other {
                    Hand::Rock => Ordering::Greater,
                    Hand::Paper => Ordering::Equal,
                    Hand::Scissor => Ordering::Less,
                }
            }
            Hand::Scissor => {
                match other {
                    Hand::Rock => Ordering::Less,
                    Hand::Paper => Ordering::Greater,
                    Hand::Scissor => Ordering::Equal
                }
            }
        }
    }
}

impl Hand {
    fn new(label: &str) -> Hand {
        return match label.to_lowercase().as_str() {
            "a" | "x" => Hand::Rock,
            "b" | "y" => Hand::Paper,
            "c" | "z" => Hand::Scissor,
            _ => panic!("letter is not recognized"),
        };
    }

    fn calculate_point(&self, other_hand: &Hand) -> (i32, i32) {
        let mut my_point = *self as i32;
        let mut other_point = *other_hand as i32;

        match self.cmp(other_hand) {
            Ordering::Less => { other_point += 6 }
            Ordering::Greater => { my_point += 6 }
            Ordering::Equal => {
                other_point += 3;
                my_point += 3;
            }
        }

        (other_point, my_point)
    }
}

fn main() {
    let contents = fs::read_to_string("src/bin/day-2/input.txt").unwrap().replace("\r\n","\n");

    //Part 1
    let matches: Vec<(Hand, Hand)> = contents
        .split('\n')
        .map(|item| -> (Hand, Hand){
            let match_str: Vec<&str> = item.split(' ').collect();
            (Hand::new(match_str.first().unwrap()),
             Hand::new(match_str.last().unwrap()))
        })
        .collect();

    let my_net_point: i32 = matches.iter().map(|item| item.1.calculate_point(&item.0).1).sum();
    println!("Part 1 answer: {}", my_net_point);

    //Part 2
    let matches: Vec<(Hand, &str)> = contents
        .split('\n')
        .map(|item| -> (Hand, &str){
            let match_str: Vec<&str> = item.split(' ').collect();
            (Hand::new(match_str.first().unwrap()),
             match_str.last().unwrap())
        })
        .collect();

    let my_net_point: i32 = matches.iter().map(|item| {
        let my_hand = match item.1.to_lowercase().as_str() {
            "x" => match item.0 {
                    Hand::Rock => Hand::Scissor,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissor => Hand::Paper,
                }
            "y" => item.0,
            "z" => match item.0 {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissor,
                Hand::Scissor => Hand::Rock,
            },
            _ => panic!("Input has flaws")
        };

        my_hand.calculate_point(&item.0).1
    }).sum();
    println!("Part 2 answer: {}", my_net_point);
}