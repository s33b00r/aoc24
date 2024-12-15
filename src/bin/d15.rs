use core::panic;
use std::{collections::{HashMap, HashSet}, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn get_dir(c: char) -> (i32, i32) {
    match c {
        '>' => (1, 0),
        '^' => (0, -1),
        '<' => (-1, 0),
        'v' => (0, 1),
        _ => panic!("Cannot handle {}", c)
    }
}

#[derive(Debug)]
enum Object {
    Wall,
    Box,
    Robot
}

#[derive(Debug)]
enum Object2 {
    Wall,
    LeftBox,
    RightBox,
    Robot
}

fn parse_map(str_map: &str) -> (HashMap<(i32, i32), Object>, (i32, i32)) {
    let mut map: HashMap<(i32, i32), Object> = HashMap::new();
    let mut robot_pos = (-1, -1);
    for (y, l) in str_map.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => { map.insert((x as i32, y as i32), Object::Wall); },
                'O' => { map.insert((x as i32, y as i32), Object::Box); },
                '@' => { map.insert((x as i32, y as i32), Object::Robot); robot_pos = (x as i32, y as i32); }
                '.' => {}
                _ => panic!("Cannot handle {}", c)
            }
        }
    }
    (map, robot_pos)
}

fn parse_map_2(str_map: &str) -> (HashMap<(i32, i32), Object2>, (i32, i32)) {
    let mut map: HashMap<(i32, i32), Object2> = HashMap::new();
    let mut robot_pos = (-1, -1);
    for (y, l) in str_map.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => { 
                    map.insert(((x * 2) as i32, y as i32), Object2::Wall); 
                    map.insert(((x * 2 + 1) as i32, y as i32), Object2::Wall); 
                },
                'O' => { 
                    map.insert(((x * 2) as i32, y as i32), Object2::LeftBox); 
                    map.insert(((x * 2 + 1) as i32, y as i32), Object2::RightBox); 
                },
                '@' => { map.insert(((x * 2) as i32, y as i32), Object2::Robot); robot_pos = ((x * 2) as i32, y as i32); }
                '.' => {}
                _ => panic!("Cannot handle {}", c)
            }
        }
    }
    (map, robot_pos)
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i32 = if !args.second {
        let (str_map, p) = args.input.split_once("\n\n").unwrap();
        let path: Vec<(i32, i32)> = p.chars().filter(|c| c != &'\n').map(get_dir).collect();
        let (mut map, mut robot_pos) = parse_map(str_map);
        for step in path {
            let new_pos = (step.0 + robot_pos.0, step.1 + robot_pos.1);
            if let None = map.get(&new_pos) {
                map.insert(new_pos, Object::Robot);
                map.remove(&robot_pos);
                robot_pos = new_pos;
                continue;
            }
            match map.get(&new_pos).unwrap() {
                Object::Wall => {},
                Object::Robot => panic!("New pos contains robot"),
                Object::Box => {
                    let mut last_box_pos = new_pos;
                    loop {
                        let p = (last_box_pos.0 + step.0, last_box_pos.1 + step.1);
                        if !map.contains_key(&p) {
                            map.insert(new_pos, Object::Robot);
                            map.insert(p, Object::Box);
                            map.remove(&robot_pos);
                            robot_pos = new_pos;
                            break;
                        }
                        match map.get(&p).unwrap() {
                            Object::Wall => break,
                            Object::Box => last_box_pos = p,
                            Object::Robot => panic!("New pos contains robot"),
                        }
                    }
                }
            }
        }
        map.iter().filter_map(|(p, o)| match o { Object::Box => Some(p), _ => None })
            .fold(0, |acc, (x, y)| acc + x + y * 100)
    } else {
        let (str_map, p) = args.input.split_once("\n\n").unwrap();
        let path: Vec<(i32, i32)> = p.chars().filter(|c| c != &'\n').map(get_dir).collect();
        let (mut map, mut robot_pos) = parse_map_2(str_map);
        'step: for step in path {
            let new_pos = (step.0 + robot_pos.0, step.1 + robot_pos.1);
            if let None = map.get(&new_pos) {
                map.insert(new_pos, Object2::Robot);
                map.remove(&robot_pos);
                robot_pos = new_pos;
                continue;
            }
            match map.get(&new_pos).unwrap() {
                Object2::Wall => {},
                Object2::Robot => panic!("New pos contains robot"),
                Object2::LeftBox | Object2::RightBox => {
                    // Move left or right
                    if step.0 != 0 {
                        let mut nr_boxes = 0;
                        let mut contains_empty = false;
                        loop {
                            if let None = map.get(&(nr_boxes * step.0 + new_pos.0, new_pos.1)) {
                                contains_empty = true;
                                break;
                            }
                            if let Some(Object2::Wall) = map.get(&(nr_boxes * step.0 + new_pos.0, new_pos.1)) {
                                break;
                            }
                            nr_boxes += 1;
                        }
                        if !contains_empty { continue; }
                        if step.0 > 0 {
                            for x in (robot_pos.0..=(robot_pos.0+nr_boxes)).rev() {
                                let o = map.remove(&(x, robot_pos.1)).unwrap();
                                map.insert((x + 1, robot_pos.1), o);
                            }
                        } else {
                            for x in (robot_pos.0 - nr_boxes)..=robot_pos.0 {
                                let o = map.remove(&(x, robot_pos.1)).unwrap();
                                map.insert((x - 1, robot_pos.1), o);
                            }
                        }

                        robot_pos = new_pos;
                        continue;
                    }
                    // Move up or down
                    let mut boxes_to_move: HashSet<(i32, i32)> = HashSet::new();
                    boxes_to_move.insert(new_pos);
                    if let Some(Object2::LeftBox) = map.get(&new_pos) {
                        boxes_to_move.insert((new_pos.0 + 1, new_pos.1));
                    } else {
                        boxes_to_move.insert((new_pos.0 - 1, new_pos.1));
                    }
                    let mut cur_y = new_pos.1;
                    loop {
                        let mut added_box = false;
                        for b in boxes_to_move.clone().iter().filter(|(_, y)| *y == cur_y) {
                            match map.get(&(b.0, b.1 + step.1)) {
                                None => {},
                                Some(Object2::Wall) => { continue 'step; }
                                Some(Object2::Robot) => { panic!("Saw bot?"); }
                                Some(Object2::LeftBox) => { added_box = true; boxes_to_move.insert((b.0, b.1 + step.1)); boxes_to_move.insert((b.0 + 1, b.1 + step.1)); }
                                Some(Object2::RightBox) => { added_box = true; boxes_to_move.insert((b.0, b.1 + step.1)); boxes_to_move.insert((b.0 - 1, b.1 + step.1)); }
                            }
                        }
                        if !added_box { break; }
                        cur_y += step.1;
                    }
                    let mut boxes = boxes_to_move.into_iter().map(|p| p).collect::<Vec<(i32, i32)>>();
                    boxes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                    if step.1 > 0 {
                        for b in boxes.iter().rev() {
                            let o = map.remove(&b).unwrap();
                            map.insert((b.0, b.1 + 1), o);
                        }
                    } else {
                        for b in boxes {
                            let o = map.remove(&b).unwrap();
                            map.insert((b.0, b.1 - 1), o);
                        }
                    }
                    map.remove(&robot_pos);
                    map.insert(new_pos, Object2::Robot);
                    robot_pos = new_pos;
                }
            }
        }
        map.iter().filter_map(|(p, o)| match o { Object2::LeftBox => Some(*p), _ => None })
            .fold(0, |acc, (x, y)| acc + x + y * 100)
    };

    result(solution, now.elapsed(), &args);
}
