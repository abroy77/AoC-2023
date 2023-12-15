use nom::bytes::complete::tag;
use nom::character::complete::{newline, one_of, u64};
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file");
    let (_, data) = parse_input(&input).unwrap();
    let solution: usize = data
        .iter()
        .map(|(symbols, nums)| count_variations(symbols, nums))
        .sum();

    println!("Solution: {:?}", solution);
}

fn count_variations(symbols: &[char], nums: &[u64]) -> usize {
    // set up base cases
    if symbols.is_empty() {
        // no .?# left in the pattern
        if nums.is_empty() {
            // no blocks of # left to matc.
            return 1; // this is a single valid variation, so return 1
        } else {
            // there are no symbols but nums wants more blocks of #. this is not valid.
            return 0;
        }
    }
    if nums.is_empty() {
        //  no more blocks of # should be in the symbols
        if symbols.contains(&'#') {
            // yikes, there are still blocks of # in the symbols. not valid. 0
            return 0;
        } else {
            // no more blocks of # in the symbols. this is a valid variation. 1
            return 1;
        }
    }

    // here if symbols and nums are not empty
    let mut count = 0;

    // have 2 options: either the first char is '.' or it is '#'
    // if it is '?', we need to call both branches
    let first_symbol = &symbols[0];

    if ['?', '.'].contains(first_symbol) {
        // we're assuming it's a '.', which has no effect on the nums, so we skip and go to the next char
        count += count_variations(&symbols[1..], &nums);
    }
    // note, these are NOT else if, that way both branches can be executed if the first symbol is '?'

    if ['?', '#'].contains(first_symbol) {
        // we're assuming it's a '#', which means we need to consume a block of nums
        // there are 3 conditions for this to be valid:
        let block_size = nums[0] as usize;
        let num_symbols = symbols.len();
        if num_symbols >= block_size && // there are enough symbols to make a block
            !symbols[1..block_size].contains(&'.')
        //  and no '.' to break the block up
        {
            if num_symbols == block_size {
                count += count_variations(&[], &nums[1..]) //  if no symbols left, call on empty slice
            } else if symbols[block_size] != '#' {
                // or the next symbol is not a '#' so we break the block after the brock length with a '.' or a '?'
                count += count_variations(&symbols[block_size + 1..], &nums[1..]);
            }
        }
    }
    count
}

fn parse_line(input: &str) -> IResult<&str, (Vec<char>, Vec<u64>)> {
    separated_pair(
        many0(one_of(".?#")),
        tag(" "),
        separated_list0(tag(","), u64),
    )(input)
}
fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<char>, Vec<u64>)>> {
    separated_list0(newline, parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_1() {
        let input = "???.### 1,1,3";
        let (_, data) = parse_line(input).unwrap();
        assert_eq!(count_variations(&data.0, &data.1), 1);
    }
    #[test]
    fn test_all_examples() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let (_, data) = parse_input(input).unwrap();
        let solution: usize = data
            .iter()
            .map(|(symbols, nums)| count_variations(symbols, nums))
            .sum();
        assert_eq!(solution, 21);
    }
}
