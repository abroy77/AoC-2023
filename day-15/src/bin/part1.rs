use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let seqs = parse_input(&contents);
    let solution = get_solution(&seqs);
    println!("Solution: {:?}", solution);
}
fn string_hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (17 * (acc + c as usize)) % 256)
}

fn get_solution(sequences: &Vec<&str>) -> usize {
    sequences.iter().map(|s| string_hash(s)).sum()
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        assert_eq!(string_hash("HASH"), 52);
    }

    #[test]
    fn test_seq() {
        let seqs = parse_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(get_solution(&seqs), 1320);
    }
}
