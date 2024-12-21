use core::panic;
use std::{collections::HashMap, i64, num::ParseIntError, result, str::FromStr, time::Instant, usize};
use regex::Regex;
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64
}

impl Computer {
    fn get_combo(&self, combo: i64) -> i64 {
        match combo {
            0..=3 => combo,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Cannot handle {}", combo)
        }
    }


    pub fn execute(&mut self, program: &Vec<i64>) -> String {
        let mut outputs: Vec<i64> = vec![];
        let mut pointer = 0;

        while pointer < program.len() {
            match program[pointer] {
                0 => { // adv
                    let combo = self.get_combo(program[pointer + 1]);
                    pointer += 2;
                    self.a = self.a / 2i64.pow(combo as u32);
                }
                1 => { // bxl
                    let literal = program[pointer + 1];
                    pointer += 2;
                    self.b = self.b ^ literal;
                }
                2 => { // bst
                    let combo = self.get_combo(program[pointer + 1]);
                    pointer += 2;
                    self.b = combo % 8;
                }
                3 => { // jnz
                    if self.a == 0 { pointer += 2; continue; }
                    pointer = program[pointer + 1] as usize;
                }
                4 => { // bxc
                    self.b = self.b ^ self.c;
                    pointer += 2;
                }
                5 => { // out
                    let combo = self.get_combo(program[pointer + 1]);
                    pointer += 2;
                    outputs.push(combo % 8);
                }
                6 => { // bdv
                    let combo = self.get_combo(program[pointer + 1]);
                    pointer += 2;
                    self.b = self.a / 2i64.pow(combo as u32);
                }
                7 => { // cdv
                    let combo = self.get_combo(program[pointer + 1]);
                    pointer += 2;
                    self.c = self.a / 2i64.pow(combo as u32);
                }
                _ => panic!("Cannot handle {}", program[pointer])
            }
        }

        outputs.iter().fold(String::from(""), |acc, x| acc + "," + x.to_string().as_str()).strip_prefix(",").unwrap().to_string()
    }
}

impl FromStr for Computer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let regex = Regex::new(r"Register .: (\d+)").unwrap();
        let a = regex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<i64>()?;
        let b = regex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<i64>()?;
        let c = regex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<i64>()?;
        Ok(Computer { a, b, c })
    }
}

fn find_a(pairs: &Vec<(Vec<i64>, i64)>, program: &Vec<i64>, partial_a: i64, current: usize) -> Option<i64> {
    if current >= program.len() { return Some(partial_a); }
    println!("{}", partial_a);
    for (partial_program, v) in pairs {
        if current == 0 && partial_program[0] == program[0] {
            if let Some(a) = find_a(pairs, program, *v, 1) {
                return Some(a);
            }
        }
        if program[current] != partial_program[0] { continue; }
        if (partial_a >> (3 * current)) == (v & 0b1111111) {
            //println!("{}, {:?}", partial_a, partial_program);
            if let Some(a) = find_a(pairs, program, partial_a | (v << (3 * current)), current + 1) {
                return Some(a);
            }
        }
        if current + 1 < program.len() && program[current + 1] != partial_program[1] { continue; }
        if (partial_a >> (3 * (current + 1))) == (v & 0b1111) {
            if let Some(a) = find_a(pairs, program, partial_a | (v << (3 * (current + 1))), current + 2) {
                return Some(a);
            }
        }
        //if current + 2 < program.len() && program[current + 2] != partial_program[2] { continue; }
        //if (partial_a >> (3 * (current + 2))) == (v & 0b1) {
        //    println!("{:b}, {:b} -> {:b}", partial_a, v, partial_a | (v << (3 * (current + 2))));
        //    if let Some(a) = find_a(pairs, program, partial_a | (v << (3 * (current + 2))), current + 3) {
        //        return Some(a);
        //    }
        //}
    }
    None
}

fn find_a_2(computer: &mut Computer, program: &Vec<i64>, current: usize, partial_a: i64) -> Option<i64> {
    let mut min: Option<i64> = None;
    computer.a = partial_a;
    let test_program = computer.execute(program).split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    if &test_program == program {
        return Some(partial_a);
    } 
    if test_program.len() > program.len() { 
        return None;
    }
    if current >= 12 {
        for p_a in 128..1024 {
            let n_a = (p_a << 38) | (partial_a & 0b11111111111111111111111111111111111111);
            computer.a = n_a;
            if &computer.execute(program).split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>() == program {
                return Some(n_a);
            }
        }
        return None;
    }
    if current == 0 {
        for p_a in 512..1024 {
            computer.a = p_a;
            if computer.execute(program).chars().next().unwrap().to_digit(10).unwrap() as i64 == program[0] {
                if let Some(a) = find_a_2(computer, program, 1, p_a) {
                    min = Some(min.unwrap_or(i64::MAX).min(a))
                }
            }
        }
    } else {
        for i in 4..8 {
            let p_a = (i << (10 + ((current as i64 - 1) * 3))) + partial_a;
            computer.a = p_a;
            if computer.execute(program).chars().skip(2 * current).next().unwrap().to_digit(10).unwrap() as i64 == program[current] {
                if let Some(a) = find_a_2(computer, program, current + 1, p_a) {
                    min = Some(min.unwrap_or(i64::MAX).min(a))
                }
            }
        }
        if current + 1 >= program.len() { return None; }
        for i in 32..64 {
            let p_a = (i << (10 + ((current as i64 - 1) * 3))) + partial_a;
            computer.a = p_a;
            let s = computer.execute(program);
            let mut iter = s.chars().skip(2 * current);
            let first = iter.next().unwrap().to_digit(10).unwrap() as i64;
            iter.next();
            let second = iter.next().unwrap().to_digit(10).unwrap() as i64;
            if program[current] == first && program[current + 1] == second {
                if let Some(a) = find_a_2(computer, program, current + 2, p_a) {
                    min = Some(min.unwrap_or(i64::MAX).min(a))
                }
            }
        }
        if current + 2 >= program.len() { return None; }
        for i in 256..512 {
            let p_a = (i << (10 + ((current as i64 - 1) * 3))) + partial_a;
            computer.a = p_a;
            let s = computer.execute(program);
            let mut iter = s.chars().skip(2 * current);
            let first = iter.next().unwrap().to_digit(10).unwrap() as i64;
            iter.next();
            let second = iter.next().unwrap().to_digit(10).unwrap() as i64;
            iter.next();
            let third = iter.next().unwrap().to_digit(10).unwrap() as i64;
            if program[current] == first && program[current + 1] == second && program[current + 2] == third {
                if let Some(a) = find_a_2(computer, program, current + 2, p_a) {
                    min = Some(min.unwrap_or(i64::MAX).min(a))
                }
            }
        }
    }
    min
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let (computer_str, tmp_program_str) = args.input.split_once("\n\n").unwrap();
    let mut computer = computer_str.parse::<Computer>().unwrap();
    let program_str = tmp_program_str.split_once(" ").unwrap().1;
    let program = program_str.split(",")
        .map(|s| s.trim_end().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let solution: String = if !args.second {
        computer.execute(&program)
    } else {
        let mut pairs: Vec<(Vec<i64>, i64)> = Vec::new();
        for a in 512..1024 {
            computer.a = a;
            let result = computer.execute(&program).split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            pairs.push((result, a));
        }
        find_a_2(&mut computer, &program, 0, 0).unwrap().to_string()
    };

    result(solution, now.elapsed(), &args);
}
