use std::{collections::HashSet, io, num::ParseIntError, str::FromStr, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Robot {
    pub p: (i32, i32),
    v: (i32, i32)
}

impl FromStr for Robot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").unwrap();
        let (p_x, p_y) = left.split_once("=").unwrap().1.split_once(",").unwrap();
        let (v_x, v_y) = right.split_once("=").unwrap().1.split_once(",").unwrap();
        let p = (p_x.parse::<i32>()?, p_y.parse::<i32>()?);
        let v = (v_x.parse::<i32>()?, v_y.parse::<i32>()?);
        Ok(Robot { p, v })
    }
}

impl Robot {
    pub fn move_robot(&mut self, size: &(i32, i32)) {
        let new_x = (self.p.0 + self.v.0).rem_euclid(size.0);
        let new_y = (self.p.1 + self.v.1).rem_euclid(size.1);
        self.p = (new_x, new_y);
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let size = if args.example { (11, 7) } else { (101, 103) };
    let solution: i32 = if !args.second {
        let mut robots: Vec<Robot> = args.input.lines().map(|l| l.parse().unwrap()).collect();
        for _ in 0..100 {
            for i in 0..robots.len() {
                robots.get_mut(i).unwrap().move_robot(&size);
            }
        }
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        for robot in robots {
            if robot.p.0 < size.0 / 2 {
                if robot.p.1 < size.1 / 2 {
                    q1 += 1;
                } else if robot.p.1 > size.1 / 2 {
                    q3 += 1;
                }
            } else if robot.p.0 > size.0 / 2 {
                if robot.p.1 < size.1 / 2 {
                    q2 += 1;
                } else if robot.p.1 > size.1 / 2 {
                    q4 += 1;
                }
            }
        }
        q1 * q2 * q3 * q4
    } else {
        let mut robots: Vec<Robot> = args.input.lines().map(|l| l.parse().unwrap()).collect();
        let mut iterations = 0;
        loop {
            for i in 0..robots.len() {
                robots.get_mut(i).unwrap().move_robot(&size);
            }
            iterations += 1;
            println!("{}", iterations);
            let set: HashSet<(i32, i32)>= robots.iter().map(|r| r.p).collect();
            let mut in_a_row = 0;
            for y in 0..size.1 {
                let mut row = 0;
                for x in 0..size.0 {
                    if set.contains(&(x, y)) {
                        row += 1;
                        print!("#");
                    } else {
                        in_a_row = in_a_row.max(row);
                        row = 0;
                        print!(".");
                    }
                }
                println!();
            }
            if in_a_row > 20 {
                let mut input = String::new();
                
                io::stdin().read_line(&mut input).expect("Failed to read line");
                if input.contains("q") {
                    break;
                }
            }
        }
        iterations
    };

    result(solution, now.elapsed(), &args);
}
