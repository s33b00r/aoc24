use core::panic;
use std::{backtrace, collections::{HashMap, HashSet, VecDeque}, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Position {
    pub x: i32,
    pub y: i32,
    pub score: i32,
    pub dir: (i32, i32)
}

fn walk(map: &HashSet<(i32, i32)>, start_pos: (i32, i32), end_pos: (i32, i32)) -> (i32, HashMap<((i32, i32), (i32, i32)), i32>) {
    let mut seen: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();
    let mut queue: VecDeque<Position> = VecDeque::new();
    queue.push_back(Position { x: start_pos.0, y: start_pos.1, score: 0, dir: (1, 0) });
    let mut smallest = i32::MAX;
    while let Some(p) = queue.pop_front() {
        if let Some(s) = seen.get(&((p.x, p.y), p.dir)) {
            if *s <= p.score { continue; }
        }
        if !map.contains(&(p.x, p.y)) { continue; }
        seen.insert(((p.x, p.y), p.dir), p.score);
        if p.x == end_pos.0 && p.y == end_pos.1 { smallest = smallest.min(p.score); continue; }
        queue.push_back(Position { x: p.x + 1, y: p.y, score: if p.dir == (1, 0) { p.score + 1 } else { p.score + 1001 }, dir: (1, 0) });
        queue.push_back(Position { x: p.x, y: p.y - 1, score: if p.dir == (0, -1) { p.score + 1 } else { p.score + 1001 }, dir: (0, -1) });
        queue.push_back(Position { x: p.x - 1, y: p.y, score: if p.dir == (-1, 0) { p.score + 1 } else { p.score + 1001 }, dir: (-1, 0) });
        queue.push_back(Position { x: p.x, y: p.y + 1, score: if p.dir == (0, 1) { p.score + 1 } else { p.score + 1001 }, dir: (0, 1) });
    }
    (smallest, seen)
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut start = (-1, -1);
    let mut end = (-1, -1);
    for (y, l) in args.input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => {}
                '.' => { map.insert((x as i32, y as i32)); },
                'S' => { start = (x as i32, y as i32); map.insert((x as i32, y as i32)); },
                'E' => { end = (x as i32, y as i32); map.insert((x as i32, y as i32)); },
                _ => panic!("Cannot handle {}", c)
            }
        }
    }

    let solution: i32 = if !args.second {
        walk(&map, start, end).0
    } else {
        let (min, walked) = walk(&map, start, end);
        let mut backtrace: VecDeque<((i32, i32), (i32, i32))> = walked.iter()
            .filter(|(p, s)| p.0 == end && **s == min).map(|(p, _)| *p).collect();
        let mut seen_pos: HashSet<(i32, i32)> = HashSet::new();
        seen_pos.insert(end);
        while let Some(p) = backtrace.pop_front() {
            seen_pos.insert(p.0);
            if p.0 == start { continue; }
            for x in walked.keys().map(|x| *x).filter(|x| x.0 == (p.0.0 - p.1.0, p.0.1 - p.1.1)) {
                if p.0.0 - x.0.0 == x.1.0 && p.0.1 - x.0.1 == x.1.1 {
                    if walked.get(&p).unwrap() - 1 == *walked.get(&x).unwrap() { backtrace.push_back(x); }
                } else if walked.get(&p).unwrap() - 1001 == *walked.get(&x).unwrap() { backtrace.push_back(x); }
            }
        }
        seen_pos.len() as i32
    };

    result(solution, now.elapsed(), &args);
}
