use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::multi::separated_list1;
use nom::IResult;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file");
    let (_, sequences) = parse_input(&input).unwrap();
    let solution = get_solution(sequences);
    println!("Solution: {}", solution);
}

fn get_solution(sequences: Vec<Vec<isize>>) -> isize {
    sequences.iter().map(|seq| solve_seq(seq)).sum()
}

fn solve_seq(seq: &Vec<isize>) -> isize {
    if seq.iter().all(|x| *x == 0) {
        return 0;
    }
    let diffs = seq
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<isize>>();

    seq.first().unwrap() - solve_seq(&diffs)
}

fn parse_seq(input: &str) -> IResult<&str, Vec<isize>> {
    let (input, nums) = separated_list1(tag(" "), i64)(input)?;
    let nums = nums.iter().map(|x| *x as isize).collect::<Vec<isize>>();
    Ok((input, nums))
}
fn parse_input(input: &str) -> IResult<&str, Vec<Vec<isize>>> {
    separated_list1(tag("\n"), parse_seq)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_seq_ex1() {
        let seq = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(solve_seq(&seq), -3);
    }

    #[test]
    fn solve_seq_ex2() {
        let seq = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(solve_seq(&seq), 0);
    }

    #[test]
    fn solve_seq_ex3() {
        let seq = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(solve_seq(&seq), 5);
    }
}
