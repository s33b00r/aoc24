use std::{num::ParseIntError, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn parse(line: &str) -> Result<(u64, Vec<u64>), ParseIntError> {
    let (wanted, nr_str) = line.split_once(":").unwrap();
    let nr: Result<Vec<u64>, ParseIntError> = nr_str.trim().split(" ").map(|x| x.parse()).collect();
    Ok((wanted.parse()?, nr?))
}

fn part_1(wanted: u64, nr: &Vec<u64>, cur_val: u64, i: usize) -> Option<u64> {
    if cur_val == wanted && i == nr.len() { return Some(wanted); }
    if i >= nr.len() || cur_val > wanted { return None; }
    if let Some(x) = part_1(wanted, nr, cur_val * nr[i], i + 1) {
        return Some(x);
    }
    if let Some(x) = part_1(wanted, nr, cur_val + nr[i], i + 1) {
        return Some(x);
    }
    None
}

fn part_2(wanted: u64, nr: &Vec<u64>, cur_val: u64, i: usize) -> Option<u64> {
    if cur_val == wanted && i == nr.len() { return Some(wanted); }
    if i >= nr.len() || cur_val > wanted { return None; }
    if let Some(x) = part_2(wanted, nr, cur_val * nr[i], i + 1) {
        return Some(x);
    }
    if let Some(x) = part_2(wanted, nr, cur_val + nr[i], i + 1) {
        return Some(x);
    }
    if let Ok(concated) = format!("{}{}", cur_val, nr[i]).parse::<u64>() {
        if let Some(x) = part_2(wanted, nr, concated, i + 1) {
            return Some(x);
        }
    }
    None
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: u64 = if !args.second {
        args.input.lines().filter_map(|l|{
            let (wanted, nr) = parse(l).unwrap();
            part_1(wanted, &nr, nr[0], 1)
        }).sum()
    } else {
        args.input.lines().filter_map(|l|{
            let (wanted, nr) = parse(l).unwrap();
            part_2(wanted, &nr, nr[0], 1)
        }).sum()
    };

    result(solution, now.elapsed(), &args);
}
