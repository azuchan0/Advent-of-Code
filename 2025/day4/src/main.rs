use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not read");
    let mut grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut result = grid.clone();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut count = 0;
            for (dr, dc) in directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    if grid[nr as usize][nc as usize] == '@' {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                result[r][c] = 'X';
            }
        }
    }

    grid = result;

    let total: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&c| *c == 'X').count())
        .sum();

    println!("{}", total);
}
