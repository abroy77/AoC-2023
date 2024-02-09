use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, character::complete, sequence::tuple, IResult};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, hailstones) = parse_input(&contents).unwrap();
    let solution = get_solution(hailstones, (200000000000000.0, 400000000000000.0));
    println!("Solution: {:?}", solution);
}

struct Hailstone {
    position: (usize, usize, usize),
    velocity: (isize, isize, isize),
}

impl Hailstone {
    fn to_2d(&self) -> Hailstone2D {
        Hailstone2D {
            position: (self.position.0, self.position.1),
            velocity: (self.velocity.0, self.velocity.1),
        }
    }
}

struct Line2D {
    a: f64,
    b: f64,
    c: f64,
}

impl Line2D {
    fn intersection(&self, other: &Line2D) -> Option<(f64, f64)> {
        if self.a * other.b == other.a * self.b {
            // lines are parallel
            return None;
        }

        Some((
            (self.c * other.b - other.c * self.b) / (self.a * other.b - other.a * self.b),
            (self.a * other.c - other.a * self.c) / (self.a * other.b - other.a * self.b),
        ))
    }
}

struct Hailstone2D {
    position: (usize, usize),
    velocity: (isize, isize),
}

impl Hailstone2D {
    fn to_line(&self) -> Line2D {
        let a = self.velocity.1 as f64;
        let b = -self.velocity.0 as f64;
        let c = a * self.position.0 as f64 + b * self.position.1 as f64;
        Line2D { a, b, c }
    }
}

fn get_solution(stones: Vec<Hailstone>, intersection_range: (f64, f64)) -> usize {
    let lines_2d: Vec<(Line2D, Hailstone2D)> = stones
        .into_iter()
        .map(|s| s.to_2d())
        .map(|s| (s.to_line(), s))
        .collect();

    let mut count = 0;
    for (i, (l1, s1)) in lines_2d.iter().enumerate() {
        for (l2, s2) in lines_2d.iter().skip(i + 1) {
            if let Some((x, y)) = l1.intersection(l2) {
                // println!("Intersection: ({}, {})", x, y);
                // check if intersection in the past

                if s1.velocity.0 * (x as isize - s1.position.0 as isize) < 0
                    || s1.velocity.1 * (y as isize - s1.position.1 as isize) < 0
                    || s2.velocity.0 * (x as isize - s2.position.0 as isize) < 0
                    || s2.velocity.1 * (y as isize - s2.position.1 as isize) < 0
                {
                    continue;
                }

                // check if intersection in range
                if x >= intersection_range.0
                    && x <= intersection_range.1
                    && y >= intersection_range.0
                    && y <= intersection_range.1
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn parse_position(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, (p0, _, p1, _, p2)) = tuple((
        complete::u64,
        tag(", "),
        complete::u64,
        tag(", "),
        complete::u64,
    ))(input)?;

    Ok((input, (p0 as usize, p1 as usize, p2 as usize)))
}

fn parse_velocity(input: &str) -> IResult<&str, (isize, isize, isize)> {
    let (input, (v0, _, v1, _, v2)) = tuple((
        complete::i64,
        tag(", "),
        complete::i64,
        tag(", "),
        complete::i64,
    ))(input)?;

    Ok((input, (v0 as isize, v1 as isize, v2 as isize)))
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let (input, (position, velocity)) =
        separated_pair(parse_position, tag(" @ "), parse_velocity)(input)?;

    Ok((input, Hailstone { position, velocity }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hailstone>> {
    separated_list1(newline, parse_hailstone)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";

        let (_, hailstones) = parse_input(input).unwrap();
        let solution = get_solution(hailstones, (7.0, 27.0));
        assert_eq!(solution, 2);
    }
}
