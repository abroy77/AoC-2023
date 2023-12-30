/// This was incredibly hard. Could not do by myself at all. required several resources
/// 1 - https://www.youtube.com/watch?v=C5wYxR6ZAPM&ab_channel=HyperNeutrino
/// 2 - https://www.youtube.com/watch?v=VTebBsfDLvE&ab_channel=UncleScientist
/// 3 - https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html
/// 4 - https://colab.research.google.com/github/derailed-dash/Advent-of-Code/blob/master/src/AoC_2023/Dazbo's_Advent_of_Code_2023.ipynb#scrollTo=3KQ3fckZiaFE
/// 5 - https://aoc.just2good.co.uk/2023/
///
///
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    i: isize,
    j: isize,
}
impl Point {
    fn new(i: isize, j: isize) -> Self {
        Self { i, j }
    }
}
struct Grid {
    points: HashSet<Point>,
    start_point: Point,
    rows: isize,
    cols: isize,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let grid = parse_input(&contents);
    let sol = get_sequence(&grid, 26501365);
    println!("Solution: {}", sol);
}

fn parse_input(input: &str) -> Grid {
    let mut points = HashSet::new();
    let mut start_point = Point::new(0, 0);
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let point = Point::new(i as isize, j as isize);
            match c {
                '.' => {
                    points.insert(point);
                }

                'S' => {
                    start_point = point;
                    points.insert(point);
                }
                _ => {}
            };
        }
    }

    Grid {
        points,
        start_point,
        rows: rows as isize,
        cols: cols as isize,
    }
}

fn get_modded_point(point: &Point, rows: &isize, cols: &isize) -> Point {
    let i = point.i.rem_euclid(*rows);
    let j = point.j.rem_euclid(*cols);

    Point::new(i, j)
}
// fn quad_solve_3_points(data: [(usize, usize); 3]) -> (isize, isize, isize) {

// }
fn solve_quad_diff(
    first_answer: isize,
    first_difference: isize,
    second_difference: isize,
) -> (isize, isize, isize) {
    let a = second_difference / 2;
    let b = first_difference - 3 * a;
    let c = first_answer - a - b;
    (a, b, c)
}

fn get_sequence(grid: &Grid, steps: isize) -> usize {
    let size = grid.rows;
    let original = steps % (2 * size);
    let increment = 2 * size;
    let answers = (0..4)
        .map(|x| {
            let segment_steps = original + increment * x;
            let num_gardens = get_num_gardens(grid, segment_steps as usize);
            num_gardens
        })
        .collect::<Vec<_>>();

    let fd = answers.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let sd = fd.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let (a, b, c) = solve_quad_diff(answers[0] as isize, fd[0] as isize, sd[0] as isize);

    //check for x=0

    println!("answers: {:?}", answers);
    println!("fd: {:?}", fd);
    println!("sd: {:?}", sd);
    println!("a: {}, b: {}, c: {}", a, b, c);

    assert_eq!(a + b + c, answers[0] as isize);

    let num_quantized_steps = steps / (2 * size) + 1;

    let ans = a * num_quantized_steps.pow(2) + b * num_quantized_steps + c;
    ans as usize
}

fn get_num_gardens(grid: &Grid, steps: usize) -> usize {
    let mut seen = HashSet::new();
    let mut ans = HashSet::new();
    let mut q = VecDeque::new();

    // get start position (i,j) where 'S' is the char in grid
    let start_point = grid.start_point;
    q.push_back((start_point, steps));

    while let Some((Point { i, j }, num_steps_left)) = q.pop_front() {
        if num_steps_left % 2 == 0 {
            ans.insert((i, j));
        }
        if num_steps_left == 0 {
            continue;
        }

        //4 directions
        let next_steps = [
            Point::new(i - 1, j),
            Point::new(i + 1, j),
            Point::new(i, j - 1),
            Point::new(i, j + 1),
        ];
        for next_step in next_steps {
            let modded_point = get_modded_point(&next_step, &grid.rows, &grid.cols);
            if !grid.points.contains(&modded_point) || seen.contains(&next_step) {
                continue;
            }
            seen.insert(next_step);
            q.push_back((next_step, num_steps_left - 1));
        }
    }
    ans.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_solution() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        let grid = parse_input(input);
        let sol = get_num_gardens(&grid, 500);
        assert_eq!(sol, 167004);
    }
}
