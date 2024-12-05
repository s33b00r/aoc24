use std::{collections::HashMap, num::ParseIntError, str::FromStr, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct PageOrder {
    pub map: HashMap<u32, Vec<u32>>,
    pub ordering: Vec<Vec<u32>>
}

impl FromStr for PageOrder {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_str, order_str) = s.split_once("\n\n").unwrap();
        let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

        for l in map_str.lines() {
            let (f_str, l_str) = l.split_once("|").unwrap();
            let first = f_str.parse::<u32>()?;
            let last = l_str.parse::<u32>()?;
            if !map.contains_key(&first) { map.insert(first, vec![]); }
            map.get_mut(&first).unwrap().push(last);
        }
        let ordering = order_str.lines()
            .map(|l| l.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
            .collect();
        Ok(PageOrder { map, ordering })
    }
}

fn is_correct_order(order_vec: &Vec<u32>, order_map: &HashMap<u32, Vec<u32>>) -> bool {
    for i in 0..order_vec.len()-1 {
        for j in i+1..order_vec.len() {
            if let Some(l) = order_map.get(&order_vec[j]) {
                if l.contains(&order_vec[i]) { return false; }
            }
        }
    }
    true
}

fn put_correct_order(order_vec: &Vec<u32>, order_map: &HashMap<u32, Vec<u32>>) -> Option<Vec<u32>> {
    let mut new_order = order_vec.clone();
    loop {
        let mut changed = false;
        for i in 0..new_order.len()-1 {
            for j in i+1..new_order.len() {
                if i == j { continue; }
                if let Some(l) = order_map.get(&new_order[j]) {
                    if l.contains(&new_order[i]) && j > i {
                        let tmp = new_order[j];
                        new_order[j] = new_order[i];
                        new_order[i] = tmp;
                        changed = true;
                        break;
                    }
                }
            }
            if changed { break; }
        }
        if changed { continue; }
        break;
    }
    if order_vec != &new_order { Some(new_order) } else { None }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: u32 = if !args.second {
        let input: PageOrder = args.input.parse().unwrap();
        input.ordering.iter()
            .filter(|l| is_correct_order(l, &input.map))
            .map(|l| l[l.len() / 2])
            .sum()
    } else {
        let input: PageOrder = args.input.parse().unwrap();
        input.ordering.iter()
            .filter_map(|l| put_correct_order(l, &input.map))
            .map(|l| l[l.len() / 2])
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
