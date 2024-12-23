use core::panic;
use std::{collections::{HashMap, HashSet, VecDeque}, str::FromStr, time::Instant};
use y24::{args, result};

struct Map {
    w: i32,
    h: i32,
    walls: HashSet<(i32, i32)>,
    pub start: (i32, i32),
    end: (i32, i32)
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut w = -1;
        let mut h = -1;
        let mut start = (-1, -1);
        let mut end = (-1, -1);
        let mut walls = HashSet::new();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                w = w.max(x as i32);
                h = h.max(y as i32);
                match c {
                    '#' => { walls.insert((x as i32, y as i32)); }
                    '.' => { }
                    'S' => start = (x as i32, y as i32),
                    'E' => end = (x as i32, y as i32),
                    _ => panic!("Cannot handle {}", c)
                }
            }
        }
        Ok(Map { w, h, walls, start, end })
    }
}

impl Map {
    pub fn get_new_pos(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut new_pos = vec![];
        for d in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
            let n_x = x + d.0;
            let n_y = y + d.1;
            if n_x < 0 || n_x > self.w || n_y < 0 || n_y > self.h { continue; }
            if self.walls.contains(&(n_x, n_y)) { continue; }
            new_pos.push((n_x, n_y));
        }
        new_pos
    }

    pub fn get_new_cheat_pos(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut new_pos = vec![];
        for d_y in -2..=2i32 {
            for d_x in -2..=2i32 {
                if d_x.abs() + d_y.abs() != 2 { continue; }
                let n_x = x + d_x;
                let n_y = y + d_y;
                if n_x < 0 || n_x > self.w || n_y < 0 || n_y > self.h { continue; }
                if self.walls.contains(&(n_x, n_y)) { continue; }
                new_pos.push((n_x, n_y));
            }
        }
        new_pos
    }

    pub fn get_new_cheat_pos_2(&self, x: i32, y: i32) -> Vec<((i32, i32), i32)> {
        let mut new_pos = vec![];
        for d_y in -20..=20i32 {
            for d_x in -20..=20i32 {
                if d_x.abs() + d_y.abs() > 20 { continue; }
                let n_x = x + d_x;
                let n_y = y + d_y;
                if n_x < 0 || n_x > self.w || n_y < 0 || n_y > self.h { continue; }
                if self.walls.contains(&(n_x, n_y)) { continue; }
                new_pos.push(((n_x, n_y), d_x.abs() + d_y.abs()));
            }
        }
        new_pos
    }

    pub fn is_end(&self, x: i32, y: i32) -> bool {
        self.end.0 == x && self.end.1 == y
    }
}

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let map = args.input.parse::<Map>().unwrap();
    let mut normal_shortest_distance = -1;
    let mut queue = VecDeque::new();
    let mut shortest = HashMap::new();
    queue.push_back((map.start, 0));
    while let Some((p, s)) = queue.pop_front() {
        if shortest.contains_key(&p) { continue; }
        shortest.insert(p, s);
        if map.is_end(p.0, p.1) { normal_shortest_distance = s; break; }
        for n_pos in map.get_new_pos(p.0, p.1) {
            queue.push_back((n_pos, s + 1));
        }
    }
    for v in shortest.values_mut() {
        *v = normal_shortest_distance - *v;
    }

    let solution: i32 = if !args.second {
        let mut nr_less = vec![];
        for (p, d) in shortest.iter() {
            for n_p in map.get_new_cheat_pos(p.0, p.1) {
                let diff = d + 2 - shortest[&n_p];
                if diff < 0 { nr_less.push(diff); }
            }
        }
        nr_less.into_iter().filter(|x| x <= &-100).count() as i32
    } else {
        let mut nr_less = vec![];
        for (p, d) in shortest.iter() {
            for n_p in map.get_new_cheat_pos_2(p.0, p.1) {
                let diff = d + n_p.1 - shortest[&n_p.0] ;
                if diff < 0 { nr_less.push(diff); }
            }
        }
        nr_less.into_iter().filter(|x| x <= &-100).count() as i32
    };

    result(solution, now.elapsed(), &args);
}
