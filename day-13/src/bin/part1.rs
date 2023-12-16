use core::num;
use core::panic;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let patterns = parse_input(&contents);
    let solution: usize = patterns.iter().map(|p| p.get_score()).sum();
    println!("Solution: {:?}", solution);
}
struct Pattern {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}
impl Pattern {
    fn new(data: Vec<Vec<bool>>) -> Self {
        // make cols
        let col_length = data[0].len();
        let cols: Vec<Vec<bool>> = (0..col_length)
            .map(|col| data.iter().map(|row| row[col]).collect::<Vec<bool>>())
            .collect();

        Pattern { rows: data, cols }
    }

    fn get_score(&self) -> usize {
        100 * find_reflection(&self.rows) + find_reflection(&self.cols)
    }
}

fn find_reflection(lines: &Vec<Vec<bool>>) -> usize {
    let num_lines = lines.len();
    for i in 1..num_lines {
        let window_length = i.min(num_lines - i);
        let h1 = &lines[(i - window_length)..i]
            .iter()
            .rev()
            .collect::<Vec<&Vec<bool>>>();

        let h2 = &lines[i..i + window_length]
            .iter()
            .collect::<Vec<&Vec<bool>>>();

        if h1 == h2 {
            return i;
        }
    }
    0
}

fn parse_pattern(input: &str) -> Pattern {
    let mut data: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let line_chars: Vec<bool> = line.chars().map(|c| c == '#').collect();
        data.push(line_chars);
    }

    Pattern::new(data)
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let pattern_strings = input.split("\n\n").collect::<Vec<&str>>();
    pattern_strings.iter().map(|s| parse_pattern(s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let pattern = parse_pattern(input);
        assert_eq!(pattern.get_score(), 5);
    }

    #[test]
    fn test_ex_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let pattern = parse_pattern(input);
        assert_eq!(pattern.get_score(), 400);
    }
}
