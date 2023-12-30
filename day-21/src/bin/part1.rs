use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let grid = parse_input(&contents);
    let sol = get_solution(grid, 64);
    println!("Solution: {}", sol);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        grid.push(row);
    }
    grid
}

fn get_start_pos(grid: &Vec<Vec<char>>) -> (isize, isize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (i as isize, j as isize);
            }
        }
    }
    panic!("No start position found");
}

fn get_solution(grid: Vec<Vec<char>>, max_steps: usize) -> usize {
    let mut seen = HashSet::new();
    let mut ans = HashSet::new();
    let mut q = VecDeque::new();
    let grid_cols = grid[0].len() as isize;
    let grid_rows = grid.len() as isize;

    // get start position (i,j) where 'S' is the char in grid
    let start_point = get_start_pos(&grid);
    q.push_back((start_point, max_steps));

    while let Some(((i, j), num_steps_left)) = q.pop_front() {
        if num_steps_left % 2 == 0 {
            ans.insert((i, j));
        }
        if num_steps_left == 0 {
            continue;
        }

        //4 directions
        let next_steps = vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        for next_step in next_steps {
            if next_step.0 < 0
                || next_step.1 < 0
                || next_step.0 >= grid_rows
                || next_step.1 >= grid_cols
                || grid[next_step.0 as usize][next_step.1 as usize] == '#'
                || seen.contains(&next_step)
            {
                continue;
            }
            seen.insert(next_step);
            q.push_back((next_step, num_steps_left - 1));
        }
    }
    ans.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_solution() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        let grid = parse_input(input);
        let sol = get_solution(grid, 6);
        assert_eq!(sol, 16);
    }
}
