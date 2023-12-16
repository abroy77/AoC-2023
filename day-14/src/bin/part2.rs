use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let cfg = parse_input(&contents);
    let cfg = find_gridstate(cfg, 1_000_000_000);
    let cfg = transpose(cfg);
    let solution = cfg.into_iter().fold(0, |acc, col| acc + col_load(col));

    println!("Solution: {:?}", solution);
}

fn roll_rocks(mut col: Vec<char>) -> Vec<char> {
    let mut empty_loc = 0;

    for i in 0..col.len() {
        match col[i] {
            '.' => continue,
            '#' => empty_loc = i + 1,
            'O' => {
                col[i] = '.';
                col[empty_loc] = 'O';
                empty_loc += 1;
            }
            _ => {}
        }
    }
    col
}

fn col_load(col: Vec<char>) -> usize {
    let col_length = col.len();
    col.into_iter().enumerate().fold(
        0,
        |acc, (i, c)| {
            if c == 'O' {
                acc + col_length - i
            } else {
                acc
            }
        },
    )
}

fn find_gridstate(mut grid: Vec<Vec<char>>, num_cycles: usize) -> Vec<Vec<char>> {
    // returns a tuple of (cycle_start, cycle_length)
    let mut seen = vec![];
    seen.push(grid.clone());

    let mut i = 0;
    let cycle_start: usize;
    loop {
        i += 1;
        grid = cycle(grid);
        if let Some(value) = seen.iter().position(|g| g == &grid) {
            cycle_start = value;
            break;
        }
        seen.push(grid.clone());
    }

    let cycle_length = i - cycle_start;
    let answer_index = (num_cycles - cycle_start) % cycle_length + cycle_start;
    seen[answer_index].clone()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn transpose<T: Copy>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let cols = input[0].len();
    let rows = input.len();

    (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).collect())
        .collect()
}
fn cycle(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..4 {
        grid = transpose(grid);
        // roll rocks
        grid = grid
            .into_iter()
            .map(|v| roll_rocks(v))
            .collect::<Vec<Vec<char>>>();

        grid = flip_horizontal(grid)
    }

    grid
}

fn flip_horizontal(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input
        .into_iter()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

fn cfg_to_string(cfg: &Vec<Vec<char>>) -> String {
    cfg.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    }
    fn cycle_1_result() -> &'static str {
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
    }

    fn cycle_2_result() -> &'static str {
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
    }
    fn cycle_3_result() -> &'static str {
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
    }

    #[test]
    fn test_parsing() {
        let input = get_input();
        let parsed = parse_input(&input);
        assert_eq!(cfg_to_string(&parsed), input);
    }
    #[test]
    fn test_cycles() {
        let input = get_input();
        let parsed = parse_input(&input);
        let cycled = cycle(parsed);
        assert_eq!(cfg_to_string(&cycled), cycle_1_result());
        let cycled = cycle(cycled);
        assert_eq!(cfg_to_string(&cycled), cycle_2_result());
        let cycled = cycle(cycled);
        assert_eq!(cfg_to_string(&cycled), cycle_3_result());

        // let load = moved.into_iter().fold(0, |acc, col| acc + col_load(col));
        // assert_eq!(load, 136);
    }
}
