use nom::bytes::complete::take;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::env;
use std::fs;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: &char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("not a valid card"),
        }
    }
}
#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(PartialEq, Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => {
                // same hand type, compare cards
                self.cards.partial_cmp(&other.cards)
            }
            Some(ord) => Some(ord),
            None => None,
        }
    }
}

impl Hand {
    fn get_hand_type(cards: &[Card; 5]) -> HandType {
        // need to define the rules for each type
        let card_counts = Hand::count_cards(cards);

        match card_counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if card_counts[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if card_counts[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
    fn count_cards(cards: &[Card; 5]) -> [u8; 13] {
        let mut card_counts = [0; 13];
        for card in cards {
            match card {
                Card::Two => card_counts[0] += 1,
                Card::Three => card_counts[1] += 1,
                Card::Four => card_counts[2] += 1,
                Card::Five => card_counts[3] += 1,
                Card::Six => card_counts[4] += 1,
                Card::Seven => card_counts[5] += 1,
                Card::Eight => card_counts[6] += 1,
                Card::Nine => card_counts[7] += 1,
                Card::Ten => card_counts[8] += 1,
                Card::Jack => card_counts[9] += 1,
                Card::Queen => card_counts[10] += 1,
                Card::King => card_counts[11] += 1,
                Card::Ace => card_counts[12] += 1,
            }
        }
        card_counts.sort();
        card_counts.reverse();
        card_counts
    }
    fn new(cards: [Card; 5]) -> Hand {
        let hand_type = Hand::get_hand_type(&cards);
        Hand { cards, hand_type }
    }
}

type Bet = (Hand, u32);
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need argument to filepath")
    }
    let data = fs::read_to_string(&args[1]).expect("file not present");

    let (_, mut bets) = parse_input(&data).unwrap();

    bets.sort_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap());

    let sol = bets.iter().enumerate().fold(0, |mut acc, (i, (_, bid))| {
        acc += bid * (i + 1) as u32;
        acc
    });

    println!("solution: {}", sol);
}

// fn parse_input(input: &str) -> IResult<&str, Vec<Bet> {
//     let mut lines = input.lines();
//     let (_, hand1) = parse_hand(lines.next().unwrap())?;
//     let (_, hand2) = parse_hand(lines.next().unwrap())?;

//     Ok((input, (hand1, hand2)))
// }

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, card_chars) = take(5usize)(input)?;
    let mut cards: [Card; 5] = [Card::Two; 5];

    let cards_vec: Vec<Card> = card_chars.chars().map(|c| Card::from_char(&c)).collect();

    // put card_vec into cards
    cards_vec
        .iter()
        .enumerate()
        .for_each(|(i, card)| cards[i] = *card);

    let hand = Hand::new(cards);
    Ok((input, hand))
}

fn parse_bid(input: &str) -> IResult<&str, u32> {
    let (input, bid) = complete::u32(input)?;
    Ok((input, bid))
}
fn parse_bet(input: &str) -> IResult<&str, Bet> {
    let (input, bet) =
        separated_pair(parse_hand, nom::bytes::complete::tag(" "), parse_bid)(input)?;
    Ok((input, bet))
}
fn parse_input(input: &str) -> IResult<&str, Vec<Bet>> {
    let (_, bets) = separated_list1(complete::line_ending, parse_bet)(input)?;
    Ok((input, bets))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_input() -> String {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string()
    }

    #[test]
    fn test_example() {
        let input = make_test_input();
        let (_, mut bets) = super::parse_input(&input).unwrap();
        bets.sort_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap());

        let sol = bets.iter().enumerate().fold(0, |mut acc, (i, (_, bid))| {
            acc += bid * (i + 1) as u32;
            acc
        });
        assert_eq!(sol, 6440);
    }

    #[test]
    fn parse_hand_test() {
        let input = "32T3K";
        let (out_str, hand) = parse_hand(input).unwrap();
        assert_eq!(
            hand,
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                hand_type: HandType::OnePair
            }
        );
        assert_eq!(out_str, "");
    }
    #[test]
    fn parse_bid_test() {
        let input = "765";
        let (out_str, bid) = parse_bid(input).unwrap();
        assert_eq!(bid, 765);
        assert_eq!(out_str, "");
    }

    #[test]
    fn parse_bet_test() {
        let input = "32T3K 765";
        let (_, bet) = parse_bet(input).unwrap();
        assert_eq!(
            bet,
            (
                Hand {
                    cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                    hand_type: HandType::OnePair
                },
                765
            )
        );
    }
}
