use std::time::Instant;
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn is_decreasing(prev: i32, cur: i32) -> bool {
    prev == -1 || (prev > cur && (prev - cur).abs() <= 3)
}

fn is_increasing(prev: i32, cur: i32) -> bool {
    prev == -1 || (prev < cur && (prev - cur).abs() <= 3)
}

fn is_increasing_or_decreasing(acc: (i32, bool, bool), n: i32) -> (i32, bool, bool) {
    (n, acc.1 && is_decreasing(acc.0, n), acc.2 && is_increasing(acc.0, n))
}

fn second(vec: &Vec<i32>) -> bool {
    for ignore in -1..(vec.len() as i32) {
        let is_correct = vec.clone().iter().enumerate()
            .filter_map(|x| if (x.0 as i32) == ignore { None } else { Some(*x.1) })
            .fold((-1, true, true), is_increasing_or_decreasing);

        if is_correct.1 || is_correct.2 { return true; }
    }
    false
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i32 = if !args.second {
        args.input.lines()
            .map(|l| l.split(" ").map(|s| s.parse::<i32>().unwrap()))
            .map(|i| i.fold((-1, true, true), is_increasing_or_decreasing))
            .fold(0, |acc, r| if r.1 || r.2 { acc + 1 } else { acc })
    } else {
        args.input.lines()
            .map(|l| l.split(" ").map(|s| s.parse::<i32>().unwrap()))
            .map(|i| second(&i.collect()))
            .filter(|r| *r)
            .count() as i32
    };

    result(solution, now.elapsed(), &args);
}
