use std::collections::VecDeque;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not read");
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
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
    let mut grid = grid;
    let mut removed = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }
            let mut count = 0;
            for (dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    if grid[nr as usize][nc as usize] == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                queue.push_back((r, c));
            }
        }
    }

    while let Some((r, c)) = queue.pop_front() {
        if grid[r][c] != '@' {
            continue;
        }
        let mut count = 0;
        for (dr, dc) in &directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
        }
        if count >= 4 {
            continue;
        }
        grid[r][c] = '.';
        removed += 1;
        for (dr, dc) in &directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == '@' {
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    println!("{}", removed);
}
