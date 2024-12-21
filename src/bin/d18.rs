use std::{collections::{HashSet, VecDeque}, time::Instant, usize};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn solve_maze(start: (i32, i32), end: (i32, i32), _blocked: &HashSet<(i32, i32)>, w: i32, h: i32) -> Option<usize> {
    let mut blocked = _blocked.clone();
    let mut to_check: VecDeque<((i32, i32), usize)> = VecDeque::new();
    to_check.push_back((start, 0));
    while let Some((pos, d)) = to_check.pop_front() {
        if blocked.contains(&pos) { continue; }
        blocked.insert(pos);
        if pos.0 == end.0 && pos.1 == end.1 { return Some(d); }
        for s in vec![(1, 0), (0, -1), (-1, 0), (0, 1)] {
            let n_x = s.0 + pos.0;
            let n_y = s.1 + pos.1;
            if n_x < 0 || n_x > w || n_y < 0 || n_y > h { continue; }
            to_check.push_back(((n_x, n_y), d + 1));
        }
    }
    None
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let (w, h) = if args.example { (6, 6) } else { (70, 70) };

    let solution: String = if !args.second {
        let nr_blocked = if args.example { 12 } else { 1024 };
        let blocked = args.input.lines().take(nr_blocked)
            .map(|s| s.split_once(",").unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect::<HashSet<(i32, i32)>>();
        solve_maze((0, 0), (w, h), &blocked, w, h).unwrap().to_string()
    } else {
        let all_blocked =  args.input.lines().map(|s| s.split_once(",").unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect::<Vec<(i32, i32)>>();
        let mut nr_to_block = if args.example { 13 } else { 1025 };
        while let Some(_) = solve_maze((0, 0), (w, h), &all_blocked.iter().take(nr_to_block).map(|p| *p).collect(), w, h) {
            nr_to_block += 1;
        }
        format!("{},{}", all_blocked[nr_to_block - 1].0, all_blocked[nr_to_block - 1].1)
    };

    result(solution, now.elapsed(), &args);
}
