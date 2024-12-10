use std::{collections::HashSet, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn find_trails_or_ratings(map: &Vec<Vec<u32>>, x: usize, y: usize, cur: u32, seen_ends: &mut Option<HashSet<(usize, usize)>>) -> u32 {
    if cur == 9 {
        match seen_ends {
            Some(seen) => {
                if !seen.contains(&(x, y)) {
                    seen.insert((x, y));
                    return 1;
                }
                return 0;
            },
            None => return 1
        }
    }
    let mut sum = 0;
    if x > 0 && map[y][x - 1] == cur + 1 { sum += find_trails_or_ratings(map, x - 1, y, cur + 1, seen_ends); }
    if x < map[0].len() - 1 && map[y][x + 1] == cur + 1 { sum += find_trails_or_ratings(map, x + 1, y, cur + 1, seen_ends); }
    if y > 0 && map[y - 1][x] == cur + 1 { sum += find_trails_or_ratings(map, x, y - 1, cur + 1, seen_ends); }
    if y < map.len() - 1 && map[y + 1][x] == cur + 1 { sum += find_trails_or_ratings(map, x, y + 1, cur + 1, seen_ends); }
    sum
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let map: Vec<Vec<u32>> = args.input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let solution: u32 = if !args.second {
        (0..map.len()).map(|y| (0..map[0].len()).map(move |x| (x, y))).flatten()
            .filter_map(|(y, x)| if map[y][x] == 0 { Some(find_trails_or_ratings(&map, x, y, 0, &mut Some(HashSet::new()))) } else { None })
            .sum()
    } else {
        (0..map.len()).map(|y| (0..map[0].len()).map(move |x| (x, y))).flatten()
            .filter_map(|(y, x)| if map[y][x] == 0 { Some(find_trails_or_ratings(&map, x, y, 0, &mut None)) } else { None })
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
