use std::{num::ParseIntError, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn parse(line: &str) -> Result<(u64, Vec<u64>), ParseIntError> {
    let (wanted, nr_str) = line.split_once(":").unwrap();
    let nr: Result<Vec<u64>, ParseIntError> = nr_str.trim().split(" ").map(|x| x.parse()).collect();
    Ok((wanted.parse()?, nr?))
}

fn concat(left: u64, right: u64) -> Option<u64> {
    let steps = ((right as f64).log10()) as u32;
    left.checked_mul(10u64.pow(steps+1))?.checked_add(right)
}

fn check(wanted: u64, nr: &Vec<u64>, cur_val: u64, i: usize, ops: &Vec<Box<dyn Fn(u64, u64) -> Option<u64>>>) -> bool {
    if i == nr.len() { return cur_val == wanted; }
    ops.iter().filter_map(|op| op(cur_val, nr[i]))
        .filter(|x| *x <= wanted)
        .map(|x| check(wanted, nr, x, i+1, ops))
        .any(|b| b)
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: u64 = if !args.second {
        args.input.lines().filter_map(|l|{
            let (wanted, nr) = parse(l).unwrap();
            if check(wanted, &nr, nr[0], 1, &vec![Box::new(u64::checked_mul), Box::new(u64::checked_add)]) { Some(wanted) } else { None }
        }).sum()
    } else {
        args.input.lines().filter_map(|l|{
            let (wanted, nr) = parse(l).unwrap();
            if check(wanted, &nr, nr[0], 1, &vec![Box::new(u64::checked_mul), Box::new(u64::checked_add), Box::new(concat)]) { Some(wanted) } else { None }
        }).sum()
    };

    result(solution, now.elapsed(), &args);
}
