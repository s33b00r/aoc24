use std::{collections::HashSet, time::Instant, usize};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn parse_input(input: &str) -> (Vec<Vec<bool>>, (i32, i32)) {
    let mut map = vec![vec![]];
    let mut pos = (-1, -1);
    for c in input.chars() {
        match c {
            '.' => map.last_mut().unwrap().push(false),
            '#' => map.last_mut().unwrap().push(true),
            '\n' => map.push(vec![]),
            '^' => {map.last_mut().unwrap().push(false); pos = ((map.last().unwrap().len() - 1) as i32, (map.len() - 1) as i32);}
            _ => panic!("Cannot handle {}", c)
        }
    }
    map.pop();
    (map, pos)
}

fn walk(pos: &(i32, i32), dir: &(i32, i32), map: &Vec<Vec<bool>>) -> Option<((i32, i32), (i32, i32))> {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if new_pos.0 < 0 || new_pos.1 < 0 || (new_pos.0 as usize) >= map[0].len() || (new_pos.1 as usize) >= map.len() { return None; }
    if map[new_pos.1 as usize][new_pos.0 as usize] { 
        let new_dir = match dir {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => panic!()
        };
        return Some((*pos, new_dir));
    }
    return Some((new_pos, *dir));
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let mut dir = (0, -1);
    let (mut map, mut pos) = parse_input(&args.input);
    let solution: i32 = if !args.second {
        let mut dir = (0, -1);
        let (map, mut pos) = parse_input(&args.input);
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        seen.insert(pos);
        loop {
            if let Some((new_pos, new_dir)) = walk(&pos, &dir, &map) {
                pos = new_pos;
                dir = new_dir;
                seen.insert(pos);
            } else {
                break;
            }
        }
        seen.len() as i32
    } else {
        let original_pos = pos;
        let original_dir = dir;
        let mut stuck = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] { continue; }
                map[y][x] = true;
                pos = original_pos;
                dir = original_dir;
                let mut seen: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
                loop {
                    if let Some((new_pos, new_dir)) = walk(&pos, &dir, &map) {
                        pos = new_pos;
                        dir = new_dir;
                        if !seen.insert((pos, dir)) {
                            stuck += 1;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                map[y][x] = false;
            }
        }
        stuck
    };

    result(solution, now.elapsed(), &args);
}
