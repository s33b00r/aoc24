use std::{collections::HashMap, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn parse(s: &str) -> (i64, i64) {
    let (l, r) = s.split_once("   ").unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i64 = if !args.second {
        let (mut l_vec, mut r_vec): (Vec<_>, Vec<_>) = args.input.lines().map(parse).unzip();
        l_vec.sort();
        r_vec.sort();
        l_vec.iter().zip(r_vec.iter()).fold(0, |acc, t| acc + (t.0 - t.1).abs())
    } else {
        let (l_vec, r_vec): (Vec<_>, Vec<_>) = args.input.lines().map(parse).unzip();
        let mut count_map: HashMap<i64, i64> = HashMap::new();
        r_vec.iter().for_each(|s| {count_map.insert(*s, *count_map.get(s).unwrap_or(&0) + 1);});
        l_vec.iter().fold(0, |acc, s| acc + s * count_map.get(s).unwrap_or(&0))
    };

    result(solution, now.elapsed(), &args);
}
