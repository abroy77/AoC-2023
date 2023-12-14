use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file");
    let map = parse_input(&input);
    let solution = get_solution(&map);
    println!("Solution: {:?}", solution);
}

struct Point {
    i: usize,
    j: usize,
}

struct Map {
    points: Vec<Point>,
    rows: usize,
    cols: usize,
}

fn parse_input(input: &str) -> Map {
    let mut points: Vec<Point> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                points.push(Point { i, j });
            }
        }
    }
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    Map { points, rows, cols }
}
fn manhattan_distance(a: &Point, b: &Point) -> usize {
    a.i.abs_diff(b.i) + a.j.abs_diff(b.j)
}

fn get_solution(map: &Map) -> usize {
    let galaxy_rows: HashSet<usize> = map.points.iter().map(|p| p.i).collect();
    let galaxy_cols: HashSet<usize> = map.points.iter().map(|p| p.j).collect();

    let empty_rows: HashSet<usize> = (0..map.rows)
        .into_iter()
        .filter(|r| !galaxy_rows.contains(r))
        .collect();

    let empty_cols: HashSet<usize> = (0..map.cols)
        .into_iter()
        .filter(|c| !galaxy_cols.contains(c))
        .collect();

    // we need to transform the points in the map,
    // to take into account expansion
    let mut new_points: Vec<Point> = Vec::new();
    for point in map.points.iter() {
        let new_rows = empty_rows.iter().filter(|r| **r < point.i).count();
        let new_cols = empty_cols.iter().filter(|c| **c < point.j).count();
        new_points.push(Point {
            i: point.i + new_rows,
            j: point.j + new_cols,
        })
    }
    // now we need the shortest distance between
    // each pair of points
    // shortest distance in this case is the manhattan distance

    new_points
        .iter()
        .combinations(2)
        .map(|points| manhattan_distance(points[0], points[1]))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn example_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let map = parse_input(input);
        let solution = get_solution(&map);
        assert_eq!(solution, 374);
    }
}
