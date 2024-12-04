use std::time::Instant;
use y24::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn find_xmas(x: usize, y: usize, grid: &Vec<Vec<char>>) -> u32 {
    if grid[y][x] != 'X' { return 0; }
    let mut nr_xmas = 0;
    if x < grid[0].len() - 3 && grid[y][x + 1] == 'M' && grid[y][x + 2] == 'A' && grid[y][x + 3] == 'S' { nr_xmas += 1; }
    if x >= 3 && grid[y][x - 1] == 'M' && grid[y][x - 2] == 'A' && grid[y][x - 3] == 'S' { nr_xmas += 1; }
    if y < grid.len() - 3 && grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S' { nr_xmas += 1; }
    if y >= 3 && grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S' { nr_xmas += 1; }
    if x < grid[0].len() - 3 && y < grid.len() - 3 && grid[y + 1][x + 1] == 'M' && grid[y + 2][x + 2] == 'A' && grid[y + 3][x + 3] == 'S' { nr_xmas += 1; }
    if x < grid[0].len() - 3 && y >= 3 && grid[y - 1][x + 1] == 'M' && grid[y - 2][x + 2] == 'A' && grid[y - 3][x + 3] == 'S' { nr_xmas += 1; }
    if x >= 3 && y < grid.len() - 3 && grid[y + 1][x - 1] == 'M' && grid[y + 2][x - 2] == 'A' && grid[y + 3][x - 3] == 'S' { nr_xmas += 1; }
    if x >= 3 && y >= 3 && grid[y - 1][x - 1] == 'M' && grid[y - 2][x - 2] == 'A' && grid[y - 3][x - 3] == 'S' { nr_xmas += 1; }
    nr_xmas
}

fn find_x_mas(x: usize, y: usize, grid: &Vec<Vec<char>>) -> u32 {
    if grid[y][x] != 'A' || y == 0 || x == 0 || y == grid.len() - 1 || x == grid[0].len() - 1 { return 0; }
    if ['M', 'S'].contains(&grid[y - 1][x - 1]) && ['M', 'S'].contains(&grid[y - 1][x + 1]) &&
        ['M', 'S'].contains(&grid[y + 1][x - 1]) && ['M', 'S'].contains(&grid[y + 1][x + 1]) &&
        grid[y - 1][x - 1] != grid[y + 1][x + 1] && grid[y - 1][x + 1] != grid[y + 1][x - 1] {

        return 1;
    }
    0
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: u32 = if !args.second {
        let grid: Vec<Vec<char>> = args.input.lines().map(|l| l.chars().collect()).collect();
        let mut nr_xmas = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                nr_xmas += find_xmas(x, y, &grid);
            }
        }
        nr_xmas
    } else {
        let grid: Vec<Vec<char>> = args.input.lines().map(|l| l.chars().collect()).collect();
        let mut nr_xmas = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                nr_xmas += find_x_mas(x, y, &grid);
            }
        }
        nr_xmas
    };

    result(solution, now.elapsed(), &args);
}
