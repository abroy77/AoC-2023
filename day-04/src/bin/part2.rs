use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need argument to filepath")
    }
    let data = fs::read_to_string(&args[1]).expect("file not present");

    let solution = get_solution(data);

    println!("Result is {solution}")
}
#[derive(Clone, Debug)]
struct Card {
    index: u32,
    matches: u32,
}
fn get_solution(data: String) -> u32 {
    let mut cards: Vec<Card> = Vec::new();
    //calculate the number of matches per game
    for (index, line) in data.lines().enumerate() {
        let (winning_numbers, my_numbers) = get_numbers(&line);
        // get the intersection of the two sets
        let match_count = winning_numbers.intersection(&my_numbers).count();
        cards.push(Card {
            index: index as u32,
            matches: match_count as u32,
        });
    }
    let max_index = cards.iter().max_by_key(|card| card.index).unwrap().index;

    let mut collected_cards: u32 = 0;
    let mut cards_to_explore: Vec<u32> = (0..max_index + 1).collect();
    // iterate through vec while not empty
    while let Some(index) = cards_to_explore.pop() {
        // for the number of matches, get teh indeces of the cards to copy
        let card = cards
            .get(index as usize)
            .expect("accessing non existent card");

        let indeces_to_copy = card.index + 1..card.index + 1 + card.matches;
        // add the card indeces to the cards to explore
        cards_to_explore.extend(indeces_to_copy);
        // increment collected cards
        collected_cards += 1;
    }

    collected_cards
}

type WinningNumbers = HashSet<u32>;
type MyNumbers = HashSet<u32>;

fn get_numbers(input_line: &str) -> (WinningNumbers, MyNumbers) {
    let game_line = input_line.split(": ").collect::<Vec<&str>>()[1]
        .split("| ")
        .collect::<Vec<&str>>();

    let winning_numbers = game_line[0]
        .split(" ")
        .filter_map(|num_str| num_str.trim().parse::<u32>().ok())
        .collect();

    let my_numbers = game_line[1]
        .split(" ")
        .filter_map(|num_str| num_str.trim().parse::<u32>().ok())
        .collect();

    (winning_numbers, my_numbers)
}

#[cfg(test)]
mod tests {
    use super::get_numbers;
    use std::collections::HashSet;

    fn make_test_line() -> String {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string()
    }

    fn make_test_data() -> String {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string()
    }

    #[test]
    fn test_get_numbers_1() {
        let test_line = make_test_line();
        let (winning_numbers, my_numbers) = get_numbers(&test_line);
        assert_eq!(winning_numbers, HashSet::from([41, 48, 83, 86, 17]));
        assert_eq!(my_numbers, HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]));
    }

    #[test]
    fn test_solution_example() {
        let test_data = make_test_data();
        let solution = super::get_solution(test_data);
        assert_eq!(solution, 30);
    }
}
