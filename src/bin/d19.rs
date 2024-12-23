use std::{collections::{HashMap, HashSet}, time::Instant, usize};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn nr_ways(wanted: &String, patterns: &HashSet<String>, max: usize, seen: &mut HashMap<String, usize>) -> usize {
    if wanted.is_empty() { return 1; }
    let mut sum = 0;
    for q in 1..=max.min(wanted.len()) {
        let (substr, rem) = wanted.split_at(q);
        if !patterns.contains(substr) { continue; }
        if !seen.contains_key(rem) {
            let tmp = nr_ways(&rem.to_string(), patterns, max, seen);
            seen.insert(rem.to_string(), tmp);
        }
        sum += seen[rem];
    }
    sum
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let (str_patterns, str_wanted) = args.input.split_once("\n\n").unwrap();
    let patterns: HashSet<String> = str_patterns.split(", ").map(|s| s.to_string()).collect();

    let wanted: Vec<String> = str_wanted.lines().map(|s| s.to_string()).collect();
    let max = patterns.iter().map(|s| s.len()).max().unwrap();

    let solution: usize = if !args.second {
        wanted.iter()
            .filter(|s| nr_ways(s, &patterns, max, &mut HashMap::new()) > 0 )
            .count()
    } else {
        wanted.iter()
            .map(|s| nr_ways(s, &patterns, max, &mut HashMap::new()))
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
