use std::{collections::HashSet, time::Instant};
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn find_trails(map: &Vec<Vec<u32>>, x: usize, y: usize, cur: u32, seen_ends: &mut HashSet<(usize, usize)>) -> u32 {
    if cur == 9 && !seen_ends.contains(&(x, y)) { 
        seen_ends.insert((x, y));
        return 1;
    }
    let mut sum = 0;
    if x > 0 && map[y][x - 1] == cur + 1 { sum += find_trails(map, x - 1, y, cur + 1, seen_ends); }
    if x < map[0].len() - 1 && map[y][x + 1] == cur + 1 { sum += find_trails(map, x + 1, y, cur + 1, seen_ends); }
    if y > 0 && map[y - 1][x] == cur + 1 { sum += find_trails(map, x, y - 1, cur + 1, seen_ends); }
    if y < map.len() - 1 && map[y + 1][x] == cur + 1 { sum += find_trails(map, x, y + 1, cur + 1, seen_ends); }
    sum
}

fn find_ratings(map: &Vec<Vec<u32>>, x: usize, y: usize, cur: u32) -> u32 {
    if cur == 9 { return 1; }
    let mut sum = 0;
    if x > 0 && map[y][x - 1] == cur + 1 { sum += find_ratings(map, x - 1, y, cur + 1); }
    if x < map[0].len() - 1 && map[y][x + 1] == cur + 1 { sum += find_ratings(map, x + 1, y, cur + 1); }
    if y > 0 && map[y - 1][x] == cur + 1 { sum += find_ratings(map, x, y - 1, cur + 1); }
    if y < map.len() - 1 && map[y + 1][x] == cur + 1 { sum += find_ratings(map, x, y + 1, cur + 1); }
    sum
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: u32 = if !args.second {
        let map: Vec<Vec<u32>> = args.input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let mut sum = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 0 {
                    sum += find_trails(&map, x, y, 0, &mut HashSet::new());
                }
            }
        }
        sum
    } else {
        let map: Vec<Vec<u32>> = args.input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let mut sum = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 0 {
                    sum += find_ratings(&map, x, y, 0);
                }
            }
        }
        sum
    };

    result(solution, now.elapsed(), &args);
}
