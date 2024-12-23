use std::{collections::{HashMap, HashSet}, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn find_largest_connection(known_connected: &HashSet<&str>, graph: &HashMap<&str, Vec<&str>>, _seen: &HashSet<&str>) -> HashSet<String> {
    let mut max: HashSet<String> = known_connected.iter().map(|s| s.to_string()).collect();
    if known_connected.len() == 1 {
        for o in graph[known_connected.iter().next().unwrap()].iter() {
            let tmp = find_largest_connection(&HashSet::from([*known_connected.iter().next().unwrap(), o]), graph, _seen);
            if tmp.len() > max.len() {
                max = tmp;
            }
        }
        return max;
    }
    let mut seen = _seen.clone();
    for o in graph[known_connected.iter().next().unwrap()].iter() {
        if known_connected.contains(o) || seen.contains(o) { continue; }
        if known_connected.iter().all(|p| graph[p].contains(o)) {
            seen.insert(o);
            let mut n_know_connected = known_connected.clone();
            n_know_connected.insert(o);
            let tmp = find_largest_connection(&n_know_connected, graph, &seen);
            if tmp.len() > max.len() {
                max = tmp;
            }
        }
    }
    max
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let mut graph = HashMap::new();
    for l in args.input.lines() {
        let (f, s) = l.split_once("-").unwrap();
        if graph.get(&f) == None {
            graph.insert(f, vec![]);
        }
        if graph.get(&s) == None {
            graph.insert(s, vec![]);
        }
        graph.get_mut(&f).unwrap().push(s);
        graph.get_mut(&s).unwrap().push(f);
    }

    let solution: String = if !args.second {
        let mut seen = HashSet::new();
        let mut total = 0;
        for (k, v) in graph.iter() {
            if k.chars().next().unwrap() != 't' { continue; }
            seen.insert(k);
            for i in 0..v.len() - 1 {
                for j in i + 1..v.len() {
                    if seen.contains(&v[i]) || seen.contains(&v[j]) { continue; }
                    if graph[v[i]].contains(&v[j]) { 
                        total += 1;
                    }
                }
            }
        }
        total.to_string()
    } else {
        let mut best = graph.keys().enumerate().map(|(i, k)| {
            println!("{}", i);
            find_largest_connection(&HashSet::from([*k]), &graph, &HashSet::new())
        }).max_by(|l, r| l.len().cmp(&r.len())).unwrap().into_iter().collect::<Vec<String>>();
        best.sort();
        best.into_iter().fold(String::new(), |acc, s| acc + "," + s.as_str())
            .strip_prefix(",").unwrap().to_string()
    };

    result(solution, now.elapsed(), &args);
}
