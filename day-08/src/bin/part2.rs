use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, newline};
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

fn get_search_length(start_node: &Parent, directions: &Vec<Direction>, map: &Map) -> usize {
    let mut current_node = *start_node;
    let mut step_count = 0;
    for direction in directions.iter().cycle() {
        let children = map.get(&current_node).unwrap();
        match direction {
            Direction::L => current_node = children.0,
            Direction::R => current_node = children.1,
        }
        step_count += 1;
        if current_node[2] == 'Z' {
            break;
        }
    }
    step_count
}

fn gcd(a: usize, b: usize) -> usize {
    // make sure a is the larger number
    if a < b {
        return gcd(b, a);
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
/// In our solution, we make an assumption, that each starting node has a unique ending node. And from explanations on
/// youtibe, we see that the path cycles such that len (A -> Z) == len (Z -> Z) on the second loop
/// so we simply need the LCM of all the path lengths
/// https://www.youtube.com/watch?v=_nnxLcrwO_U&t=338s&ab_channel=HyperNeutrino
///
fn get_solution(directions: Vec<Direction>, map: Map) -> usize {
    // find all the nodes that end with 'A'
    let start_nodes: Vec<Parent> = map
        .keys()
        .into_iter()
        .filter(|k| k[2] == 'A')
        .map(|x| x.clone())
        .collect();
    // let mut current_nodes = vec![];

    // for each node, find the shortest path to target
    let search_lengths: Vec<usize> = start_nodes
        .iter()
        .map(|node| get_search_length(node, &directions, &map))
        .collect();

    // find the LCM of all the path lengths
    search_lengths.iter().fold(1, |acc, x| lcm(acc, *x))
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
    let (input, parent_str) = alphanumeric1(input)?;
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
        let parent_str = "11A";
        assert_eq!(parse_node_name(parent_str), Ok(("", ['1', '1', 'A'])));
    }

    #[test]
    fn test_parse_children() {
        let children_str = "(11B, XXX)";
        assert_eq!(
            parse_children(children_str),
            Ok(("", (['1', '1', 'B'], ['X', 'X', 'X'])))
        );
    }

    #[test]
    fn test_parse_node() {
        let node_str = "22B = (22C, 22C)";
        assert_eq!(
            parse_node(node_str),
            Ok(("", (['2', '2', 'B'], (['2', '2', 'C'], ['2', '2', 'C']))))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (_, (directions, map)) = parse_input(input).unwrap();
        assert_eq!(directions, vec![Direction::L, Direction::R]);
        assert_eq!(map.len(), 8);
    }

    #[test]
    fn test_solution() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (_, (directions, map)) = parse_input(input).unwrap();
        assert_eq!(get_solution(directions, map), 6);
    }
}
