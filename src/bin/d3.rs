use std::time::Instant;
use regex::Regex;
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i32 = if !args.second {
        let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
        re.captures_iter(&args.input).map(|s| {
            let (_, [num]) = s.extract();
            num.split(",").fold(1, |acc, x| acc * x.parse::<i32>().unwrap())
        }).sum()
    } else {
        let mul_re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
        let mul_iter = mul_re.captures_iter(&args.input)
            .map(|c| {
                let start = *&c.get(0).unwrap().start();
                let (_, [num]) = c.extract();
                (start, num.split(",").fold(1, |acc, x| acc * x.parse::<i32>().unwrap()))
            });
        let do_re = Regex::new(r"do\(\)").unwrap();
        let mut do_iter = do_re.captures_iter(&args.input).map(|c| c.get(0).unwrap().start());
        let dont_re = Regex::new(r"don't\(\)").unwrap();
        let mut dont_iter = dont_re.captures_iter(&args.input).map(|c| c.get(0).unwrap().start());
        let mut ok_mul: Vec<i32> = vec![];
        let mut next_do = do_iter.next().unwrap_or(0);
        let mut next_dont = dont_iter.next().unwrap();
        for (p, v) in mul_iter {
            loop {
                if next_do < next_dont {
                    if let Some(x) = do_iter.next() {
                        next_do = x;
                    } else {
                        break;
                    }
                } else { break; }
            }
            if next_dont > p || p > next_do {
                ok_mul.push(v);
            }
            if p > next_do {
                loop {
                    if next_do > next_dont {
                        if let Some(x) = dont_iter.next() {
                            next_dont = x;
                        } else {
                            break;
                        }
                    } else { break; }
                }
            }
        }

        //do_iter.for_each(|s| println!("{:?}", s));
        ok_mul.iter().sum()
    };

    result(solution, now.elapsed(), &args);
}
