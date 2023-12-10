use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
    L = 0,
    R = 1,
}
type Children = ([char; 3], [char; 3]);
type Parent = [char; 3];
type Map = HashMap<Parent, Children>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file");
    let (_, (directions, map)) = parse_input(&input).unwrap();
    let solution = get_solution(directions, map);
    println!("Solution: {}", solution);
}

fn get_solution(directions: Vec<Direction>, map: Map) -> usize {
    let mut current_node = ['A'; 3];
    let mut step_count = 0;
    for direction in directions.iter().cycle() {
        let children = map.get(&current_node).unwrap();
        match direction {
            Direction::L => current_node = children.0,
            Direction::R => current_node = children.1,
        }
        step_count += 1;
        if current_node == ['Z'; 3] {
            break;
        }
    }
    step_count
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, direction_str) = alpha1(input)?;
    let directions = direction_str
        .chars()
        .map(|c| match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Invalid direction"),
        })
        .collect();
    Ok((input, directions))
}

fn parse_node_name(input: &str) -> IResult<&str, Parent> {
    let mut parent = ['A'; 3];
    let (input, parent_str) = alpha1(input)?;
    for (i, c) in parent_str.chars().enumerate() {
        parent[i] = c;
    }

    Ok((input, parent))
}

fn parse_children(input: &str) -> IResult<&str, Children> {
    delimited(
        tag("("),
        separated_pair(parse_node_name, tag(", "), parse_node_name),
        tag(")"),
    )(input)
}

fn parse_node(input: &str) -> IResult<&str, (Parent, Children)> {
    separated_pair(parse_node_name, tag(" = "), parse_children)(input)
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<(Parent, Children)>> {
    separated_list1(newline, parse_node)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, Map)> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    let mut map = HashMap::new();
    let (_, nodes) = parse_nodes(input)?;
    for (parent, children) in nodes {
        map.insert(parent, children);
    }
    Ok(("", (directions, map)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directions() {
        let directions = vec![Direction::L, Direction::R];
        let dir_str = "LR";
        assert_eq!(parse_directions(dir_str), Ok(("", directions)));
    }

    #[test]
    fn test_parse_node_name() {
        let parent_str = "ABC";
        assert_eq!(parse_node_name(parent_str), Ok(("", ['A', 'B', 'C'])));
    }

    #[test]
    fn test_parse_children() {
        let children_str = "(BBB, CCC)";
        assert_eq!(
            parse_children(children_str),
            Ok(("", (['B', 'B', 'B'], ['C', 'C', 'C'])))
        );
    }

    #[test]
    fn test_parse_node() {
        let node_str = "XYC = (BBB, CCC)";
        assert_eq!(
            parse_node(node_str),
            Ok(("", (['X', 'Y', 'C'], (['B', 'B', 'B'], ['C', 'C', 'C']))))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let (_, (directions, map)) = parse_input(input).unwrap();
        assert_eq!(directions, vec![Direction::R, Direction::L]);
        assert_eq!(map.len(), 7);
    }

    #[test]
    fn test_solution() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (_, (directions, map)) = parse_input(input).unwrap();
        assert_eq!(get_solution(directions, map), 6);
    }
}
