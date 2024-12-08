use std::{collections::{HashMap, HashSet}, str::FromStr, time::Instant};
use y24::{args, result, structs::Point};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Input {
    pub elements: HashMap<char, Vec<Point<i32>>>,
    pub width: i32,
    pub height: i32
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = 0;
        let mut width = 0;
        let mut elements: HashMap<char, Vec<Point<i32>>> = HashMap::new();
        s.lines().enumerate().for_each(|(y, l)| {
            height = height.max((y + 1) as i32);
            width = l.len() as i32;
            l.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    elements.entry(c).or_insert(vec![]).push(Point { x: x as i32, y: y as i32 })
                }
            });
        });

        Ok(Input { width, height, elements })
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp%b;
    }
    return a
}

fn check_boundary(pos: &Point<i32>, width: i32, height: i32) -> Option<Point<i32>> {
    if pos.x < 0 || pos.x >= width || pos.y < 0 || pos.y >= height { return None; }
    Some(pos.clone())
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i32 = if !args.second {
        let mut anti_nodes: HashSet<Point<i32>> = HashSet::new();
        let input = args.input.parse::<Input>().unwrap();

        for positions in input.elements.values() {
            for i in 0..positions.len() - 1 {
                for j in i+1..positions.len() {
                    let diff = positions[i] - positions[j];
                    if let Some(p) = check_boundary(&(positions[i] + diff), input.width, input.height) {
                        anti_nodes.insert(p);
                    }
                    if let Some(p) = check_boundary(&(positions[j] - diff), input.width, input.height) {
                        anti_nodes.insert(p);
                    }
                }
            }
        }
        anti_nodes.len() as i32
    } else {
        let mut anti_nodes: HashSet<Point<i32>> = HashSet::new();
        let input = args.input.parse::<Input>().unwrap();

        for positions in input.elements.values() {
            for i in 0..positions.len() - 1 {
                for j in i+1..positions.len() {
                    let mut diff = positions[i] - positions[j];
                    diff = diff / gcd(diff.x, diff.y);
                    let mut pos = positions[i];
                    while let Some(p) = check_boundary(&pos, input.width, input.height) {
                        anti_nodes.insert(p);
                        pos = pos + diff;
                    }
                    pos = positions[i];
                    while let Some(p) = check_boundary(&pos, input.width, input.height) {
                        anti_nodes.insert(p);
                        pos = pos - diff;
                    }
                }
            }
        }
        anti_nodes.len() as i32
    };

    result(solution, now.elapsed(), &args);
}
