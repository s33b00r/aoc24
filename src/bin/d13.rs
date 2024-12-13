use std::{num::ParseIntError, str::FromStr, time::Instant};
use regex::Regex;
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct ClawMachine {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub prize: (i64, i64)
}

impl FromStr for ClawMachine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_x = Regex::new(r"X.(\d+)").unwrap();
        let re_y = Regex::new(r"Y.(\d+)").unwrap();
        let mut lines = s.lines();
        let mut n = lines.next().unwrap();
        let a = (re_x.captures(n).unwrap().get(1).unwrap().as_str().parse::<i64>()?, 
            re_y.captures(n).unwrap().get(1).unwrap().as_str().parse::<i64>()?);
        n = lines.next().unwrap();
        let b = (re_x.captures(n).unwrap().get(1).unwrap().as_str().parse::<i64>()?, 
            re_y.captures(n).unwrap().get(1).unwrap().as_str().parse::<i64>()?);
        n = lines.next().unwrap();
        let prize = (re_x.captures(n).unwrap().get(1).unwrap().as_str().parse::<i64>()?, 
            re_y.captures(n).unwrap().get(1).unwrap().as_str().parse::<i64>()?);
        Ok(ClawMachine { a, b, prize })
    }
}

// X = A.X * a + B.X * b
// Y = A.Y * a + B.Y * b
//
// b * B.Y = Y - A.Y * a
// b = (Y - A.Y * a) / B.Y
// X = A.X * a + B.X * (Y - A.Y * a) / B.Y
// A.X * a - A.Y * a * B.X / B.Y = X - Y * B.X / B.Y
// a * (A.X - A.Y * B.X / B.Y) = _
// a = (X - Y * B.X / B.Y) / (A.X - A.Y * B.X / B.Y)
fn find_solution(machine: ClawMachine, extra: i64) -> i64 {
    let mut a_f64 = (machine.prize.0 + extra) as f64 - (machine.prize.1 + extra) as f64 * machine.b.0 as f64 / machine.b.1 as f64;
    a_f64 /= machine.a.0 as f64 - machine.a.1 as f64 * machine.b.0 as f64 / machine.b.1 as f64;
    let a = a_f64.round() as i64;
    if (a as f64 - a_f64).abs() > 0.01 { return 0; }
    let b_f64 = ((machine.prize.1 + extra) as f64 - machine.a.1 as f64 * a as f64) / machine.b.1 as f64;
    let b = b_f64.round() as i64;
    if (b as f64 - b_f64).abs() > 0.01 { return 0; }
    return a * 3 + b;
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i64 = if !args.second {
        args.input.split("\n\n")
            .map(|s| s.parse::<ClawMachine>().unwrap())
            .map(|c| find_solution(c, 0))
            .sum()
    } else {
        args.input.split("\n\n")
            .map(|s| s.parse::<ClawMachine>().unwrap())
            .map(|c| find_solution(c, 1_0000_000_000_000))
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
