use std::time::Instant;
use regex::Regex;
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();

    let solution: i32 = if !args.second {
        re.captures_iter(&args.input).map(|s| {
            let (_, [num]) = s.extract();
            num.split(",").fold(1, |acc, x| acc * x.parse::<i32>().unwrap())
        }).sum()
    } else {
        let input = "do()".to_string() + &args.input;
        let new_input = input.split("don't()")
            .filter_map(|s| s.split_once("do()").map(|x| x.1))
            .collect::<String>();
        re.captures_iter(&new_input).map(|s| {
            let (_, [num]) = s.extract();
            num.split(",").fold(1, |acc, x| acc * x.parse::<i32>().unwrap())
        }).sum()
    };

    result(solution, now.elapsed(), &args);
}
