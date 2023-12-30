use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, bricks) = parse_input(&contents).unwrap();
    let sol = get_solution(bricks);
    println!("Solution: {}", sol);
}
#[derive(Debug)]
struct Brick {
    start: (usize, usize, usize), // (x,y,z)
    end: (usize, usize, usize),   // start.z <= end.z
}

impl Brick {
    fn _range_overlap(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
        a_start.max(b_start) <= a_end.min(b_end)
    }

    fn overlap(&self, other: &Brick) -> bool {
        // only for x, y coords
        Self::_range_overlap(self.start.0, self.end.0, other.start.0, other.end.0)
            && Self::_range_overlap(self.start.1, self.end.1, other.start.1, other.end.1)
    }

    fn supports(&self, other: &Brick) -> bool {
        //if overlaps and max self.z +1 == min other.z
        self.overlap(other) && self.end.2 + 1 == other.start.2
    }
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    // sort bricks by their lowest z coord ie start.z
    bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));

    // make bricks fall into place
    for i in 0..bricks.len() {
        let mut max_z = 1;
        for check in 0..i {
            if bricks[i].overlap(&bricks[check]) {
                max_z = max_z.max(bricks[check].end.2 + 1);
            }
        }
        // update brick's z coords
        let height = bricks[i].end.2 - bricks[i].start.2;
        bricks[i].end.2 = max_z + height;
        bricks[i].start.2 = max_z;
    }
}

fn get_solution(mut bricks: Vec<Brick>) -> usize {
    // settle bricks
    settle_bricks(&mut bricks);
    let mut supports: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    let mut supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];

    // find which bricks support which bricks and
    // which bricks are supported by which
    for higher_index in 0..bricks.len() {
        for lower_index in 0..higher_index {
            if bricks[lower_index].supports(&bricks[higher_index]) {
                supports[lower_index].insert(higher_index);
                supported_by[higher_index].insert(lower_index);
            }
        }
    }

    // we can only disintegrate bricks where all the bricks it supports,
    // have other supports
    (0..bricks.len())
        .into_iter()
        .filter(|i| supports[*i].iter().all(|j| supported_by[*j].len() > 1))
        .count()
}

fn parse_coord(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, (x, _, y, _, z)) = tuple((
        complete::u64,
        tag(","),
        complete::u64,
        tag(","),
        complete::u64,
    ))(input)?;
    Ok((input, (x as usize, y as usize, z as usize)))
}
fn parse_brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start_coord, end_coord)) =
        separated_pair(parse_coord, tag("~"), parse_coord)(input)?;
    Ok((
        input,
        Brick {
            start: start_coord,
            end: end_coord,
        },
    ))
}
fn parse_input(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(newline, parse_brick)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_solution() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let (_, bricks) = parse_input(input).unwrap();
        let sol = get_solution(bricks);
        assert_eq!(sol, 5);
    }
}
