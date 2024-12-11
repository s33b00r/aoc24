use std::{collections::HashMap, time::Instant, usize};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn find_quantity(_stones: &Vec<u64>, blinks: usize) -> u64 {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    _stones.iter().for_each(|x| {stones.insert(*x, stones.get(&x).unwrap_or(&0) + 1);});
    for _ in 0..blinks {
        let mut new_stones = HashMap::new();
        for (stone, quantity) in stones {
            if stone == 0 {
                new_stones.insert(1, new_stones.get(&1).unwrap_or(&0) + quantity);
            } else if stone.to_string().len() % 2 == 0 {
                let as_str = stone.to_string();
                let (left, right) = as_str.split_at(as_str.len() / 2);
                let left_u64 = left.parse::<u64>().unwrap();
                let right_u64 = right.parse::<u64>().unwrap();
                new_stones.insert(left_u64, new_stones.get(&left_u64).unwrap_or(&0) + quantity);
                new_stones.insert(right_u64, new_stones.get(&right_u64).unwrap_or(&0) + quantity);
            } else {
                new_stones.insert(stone * 2024, new_stones.get(&(stone * 2024)).unwrap_or(&0) + quantity);
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let stones: Vec<u64> = args.input.split(" ")
        .map(|x| x.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap())
        .collect();

    let solution: u64 = if !args.second {
        find_quantity(&stones, 25)
    } else {
        find_quantity(&stones, 75)
    };

    result(solution, now.elapsed(), &args);
}
