use std::{collections::HashSet, time::Instant, usize};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug, Clone, Copy)]
struct Layout {
    pub index: usize,
    pub size: usize,
    pub empty_after: usize
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: usize = if !args.second {
        let mut compacted: Vec<usize> = vec![];
        let input = args.input.chars().collect::<Vec<char>>();
        let mut l_i = 0;
        let mut r_i = if (input.len() - 1) % 2 == 0 { input.len() - 1 } else { input.len() - 2 };
        let mut taken = 0;
        while l_i < r_i {
            for _ in 0..input[l_i].to_digit(10).unwrap() {
                if l_i % 2 == 0 {
                    compacted.push(l_i / 2);
                } else {
                    if taken >= input[r_i].to_digit(10).unwrap() {
                        loop {
                            r_i -= 2;
                            if r_i <= l_i || input[r_i] != '0' { break; }
                        }
                        taken = 0;
                    }
                    compacted.push(r_i / 2);
                    taken += 1;
                }
            }
                l_i += 1;
        }
        while taken < input[r_i].to_digit(10).unwrap() {
            compacted.push(r_i / 2);
            taken += 1;
        }
        compacted.iter().enumerate().map(|(i, x)| i * x).sum()    
    } else {
        let mut layout: Vec<Layout> = vec![];
        for (i, c) in args.input.chars().filter(|c| c.is_digit(10)).enumerate() {
            if i % 2 == 0 {
                layout.push(Layout { index: i / 2, size: c.to_digit(10).unwrap() as usize, empty_after: 0 })
            } else {
                layout.last_mut().unwrap().empty_after = c.to_digit(10).unwrap() as usize;
            }
        }
        let mut seen: HashSet<usize> = HashSet::new();
        let mut r_i = layout.len() - 1;
        'next: while r_i > 0 {
            if seen.contains(&layout[r_i].index) { r_i -= 1; continue; }
            let mut l_i = 0;
            while l_i < r_i {
                if layout[r_i].size <= layout[l_i].empty_after {
                    seen.insert(layout[r_i].index);
                    layout.get_mut(r_i - 1).unwrap().empty_after += layout[r_i].size + layout[r_i].empty_after;
                    let diff = layout[l_i].empty_after - layout[r_i].size;
                    layout.get_mut(l_i).unwrap().empty_after = 0;
                    let mut new_layout = layout.remove(r_i).clone();
                    new_layout.empty_after = diff;
                    layout.insert(l_i + 1, new_layout);
                    continue 'next;
                }
                l_i += 1;
            }
            r_i -= 1;
        }
        layout.iter().fold((0, 0), |(i, s), l| (i + l.empty_after + l.size, s + (i * l.size + (l.size * (l.size - 1)) / 2) * l.index)).1
    };
    result(solution, now.elapsed(), &args);
}
