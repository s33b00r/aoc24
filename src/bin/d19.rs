use core::panic;
use std::{collections::{HashMap, HashSet}, time::Instant, usize};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn cross_products(l: &Vec<Vec<String>>, r: &Vec<Vec<String>>) -> HashSet<Vec<String>> {
    let mut cross = HashSet::new();
    for l_item in l {
        for r_item in r {
            let mut cloned = l_item.clone();
            cloned.append(&mut r_item.clone());
            cross.insert(cloned);
        }
    }
    cross
}

fn can_get_wanted(wanted: &String, _patterns: &HashMap<String, Vec<Vec<String>>>) -> Vec<Vec<String>> {
    let mut patterns = _patterns.clone();
    for i in 0..wanted.len() {
        let partial = wanted.chars().take(i + 1).collect::<String>();
        if patterns.contains_key(&partial) { continue; }
        let mut ways: HashSet<Vec<String>> = HashSet::new();
        for j in 1..partial.len() {
            let l = patterns.get(&partial.chars().take(j).collect::<String>());
            let r = patterns.get(&partial.chars().skip(j).collect::<String>());
            if l.is_none() || r.is_none() { continue; }
            for v in cross_products(&l.unwrap(), &r.unwrap()) {
                ways.insert(v);
            }
        }
        patterns.insert(partial, Vec::from_iter(ways.into_iter()));
    }
    patterns.get(wanted).unwrap_or(&vec![]).clone()
}

fn part_2(wanted: &String, patterns: &HashSet<String>, max: usize, seen: &mut HashMap<String, usize>) -> usize {
    if wanted.is_empty() { return 1; }
    let mut sum = 0;
    for q in 1..=max.min(wanted.len()) {
        let (substr, rem) = wanted.split_at(q);
        if !patterns.contains(substr) { continue; }
        if !seen.contains_key(rem) {
            let tmp = part_2(&rem.to_string(), patterns, max, seen);
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
    let mut _patterns: Vec<String> = str_patterns.split(", ").map(|s| s.to_string()).collect();
    let mut patterns: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    _patterns.sort_by(|l, r| l.len().cmp(&r.len()));
    for p in _patterns {
        if p.len() == 0 {
            patterns.insert(p.clone(), vec![vec![p]]);
        } else {
            let mut tmp_vec = vec![vec![p.clone()]];
            tmp_vec.append(&mut can_get_wanted(&p, &patterns));
            patterns.insert(p, tmp_vec);
        }
    }

    let wanted: Vec<String> = str_wanted.lines().map(|s| s.to_string()).collect();

    let solution: usize = if !args.second {
        wanted.iter().enumerate()
            .filter(|(i, s)| {
                println!("{}", i);
                can_get_wanted(s, &patterns).len() > 0
            })
            .count()
    } else {
        let max = patterns.keys().map(|s| s.len()).max().unwrap();
        wanted.iter().enumerate()
            .map(|(i, s)| {
                println!("{}", i);
                part_2(s, &patterns.keys().map(|s| s.clone()).collect(), max, &mut HashMap::new())
            })
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
