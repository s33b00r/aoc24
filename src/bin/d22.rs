use std::{collections::VecDeque, time::Instant, u64};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn get_next(_nr: u64, steps: usize) -> u64 {
    let mut nr = _nr;
    for _ in 0..steps {
        // Step 1
        let mut partial = ((nr * 64) ^ nr) % 16777216;
        // Step 2
        partial = ((partial / 32) ^ partial) % 16777216;
        // Step 3
        partial = ((partial * 2048) ^ partial) % 16777216;
        nr = partial
    }
    nr
}

fn get_nr_bananas(_nr: u64, wanted_sequence: &[i32; 4]) -> u64 {
    let mut sequence: VecDeque<i32> = VecDeque::new();
    let mut nr = _nr;
    for _ in 0..2000 {
        let tmp = nr;
        nr = get_next(nr, 1);
        sequence.push_back((nr % 10) as i32 - (tmp % 10) as i32);
        if sequence.len() < 4 { continue; }
        if sequence.len() > 4 { sequence.pop_front(); }
        if wanted_sequence.iter().zip(sequence.iter()).all(|(w, s)| w == s) { 
            return nr % 10;
        }
    }
    0
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let input: Vec<u64> = args.input.lines().map(|s| s.parse::<u64>().unwrap()).collect();

    let solution: u64 = if !args.second {
        input.iter().map(|n| get_next(*n, 2000)).sum()
    } else {
        let mut possible_combinations: Vec<[i32; 4]> = Vec::new();
        for i in -9..=9 {
            for j in -9..=9 {
                if i + j > 9 || i + j < -9 { continue; }
                for k in -9..=9 {
                    if i + j + k > 9 || i + j + k < -9 { continue; }
                    for l in -9..=9 {
                        if i + j + k + l > 9 || i + j + k + l < -9 { continue; }
                        possible_combinations.push([i, j, k, l]);
                    }
                }
            }
        }

        let mut max_bananas = 0;
        for (i, c) in possible_combinations.iter().enumerate() {
            let mut cur_sum = 0;
            println!("{}", i);
            for i in 0..input.len() {
                cur_sum += get_nr_bananas(input[i], c);
                if (cur_sum + 9 * (input.len() - i) as u64) < max_bananas { break; }
            }
            max_bananas = max_bananas.max(cur_sum);
        }
        max_bananas
    };

    result(solution, now.elapsed(), &args);
}
