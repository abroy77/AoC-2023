use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alphanumeric1, anychar, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::fs;
use std::{collections::HashMap, env};
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, data) = parse_input(&contents).unwrap();
    let solution = get_solution(&data);
    println!("Solution: {:?}", solution);
}

static DIRECTIONS: [(char, (i64, i64)); 4] =
    [('R', (0, 1)), ('D', (1, 0)), ('L', (0, -1)), ('U', (-1, 0))];

fn get_solution(data: &Vec<(char, usize, &str)>) -> usize {
    let mut points = vec![(0, 0)];
    let mut boundary_points: i64 = 0;

    for (_, _, hex_code) in data {
        let steps = u32::from_str_radix(&hex_code[0..5], 16).unwrap();
        let (_, (di, dj)) =
            DIRECTIONS[hex_code.chars().nth(5).unwrap().to_digit(10).unwrap() as usize];

        let current_point = points.last().unwrap();
        let next_point = (
            current_point.0 + di * steps as i64,
            current_point.1 + dj * steps as i64,
        );
        boundary_points += steps as i64;
        points.push(next_point);
    }

    // use shoelace and pick's algorithm to get area of the closed integer polygon
    // add last element to front and first element to end for wrapping
    let last_point = vec![points.last().unwrap().clone()];
    let first_point = vec![points.first().unwrap().clone()];
    let points = last_point
        .iter()
        .chain(points.iter())
        .chain(first_point.iter())
        .collect::<Vec<_>>();

    let shoelace_area = (1..points.len() - 1)
        .into_iter()
        .map(|i| points[i].0 * (points[i + 1].1 - points[i - 1].1))
        .sum::<i64>()
        .abs()
        / 2;

    // using picks to get num of interior points
    let interior_points = shoelace_area + 1 - boundary_points / 2;

    (interior_points + boundary_points) as usize
}

fn parse_line(input: &str) -> IResult<&str, (char, usize, &str)> {
    let (input, data) = tuple((
        anychar,
        space1,
        nom::character::complete::u32,
        space1,
        delimited(tag("(#"), alphanumeric1, tag(")")),
    ))(input)?;

    data.0;

    Ok((input, (data.0, data.2 as usize, data.4)))
}
fn parse_input(input: &str) -> IResult<&str, Vec<(char, usize, &str)>> {
    separated_list1(tag("\n"), parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_solution() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let (_, data) = parse_input(input).unwrap();
        assert_eq!(get_solution(&data), 952408144115);
    }
}
