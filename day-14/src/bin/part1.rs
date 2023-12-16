use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let cfg = parse_input(&contents);
    let cfg = transpose(cfg);
    let cfg: Vec<Vec<char>> = cfg.into_iter().map(|v| roll_rocks(v)).collect();
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
    fn moved_north() -> &'static str {
        "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
    }

    #[test]
    fn test_parsing() {
        let input = get_input();
        let parsed = parse_input(&input);
        assert_eq!(cfg_to_string(&parsed), input);
    }
    #[test]
    fn test_move_north() {
        let input = get_input();
        let parsed = parse_input(&input);
        let transposed = transpose(parsed);
        let moved: Vec<Vec<char>> = transposed.into_iter().map(|v| roll_rocks(v)).collect();
        let move_transposed = transpose(moved.clone());
        assert_eq!(cfg_to_string(&move_transposed), moved_north());
        let load = moved.into_iter().fold(0, |acc, col| acc + col_load(col));
        assert_eq!(load, 136);
    }
}
