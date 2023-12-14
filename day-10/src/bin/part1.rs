use std::collections::HashMap;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file");
    let (map, start_point) = parse_input(&input);
    println!("rows {}\ncolumns {}", map.rows, map.cols);
    let solution = traverse(&map, start_point);
    println!("start position = {:?}", start_point);
    println!("Solution: {:?}", solution.unwrap());
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Direction {
    del_i: isize,
    del_j: isize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pipe(&'static Direction, &'static Direction);

static UP: Direction = Direction {
    del_i: -1,
    del_j: 0,
};

static DOWN: Direction = Direction { del_i: 1, del_j: 0 };

static LEFT: Direction = Direction {
    del_i: 0,
    del_j: -1,
};

static RIGHT: Direction = Direction { del_i: 0, del_j: 1 };

static PIPE_UP_DOWN: Pipe = Pipe(&UP, &DOWN);
static PIPE_UP_LEFT: Pipe = Pipe(&UP, &LEFT);
static PIPE_UP_RIGHT: Pipe = Pipe(&UP, &RIGHT);
static PIPE_LEFT_RIGHT: Pipe = Pipe(&LEFT, &RIGHT);
static PIPE_DOWN_LEFT: Pipe = Pipe(&DOWN, &LEFT);
static PIPE_DOWN_RIGHT: Pipe = Pipe(&DOWN, &RIGHT);

static SYMBOLS: [(char, &Pipe); 6] = [
    ('J', &PIPE_UP_LEFT),
    ('|', &PIPE_UP_DOWN),
    ('L', &PIPE_UP_RIGHT),
    ('F', &PIPE_DOWN_RIGHT),
    ('-', &PIPE_LEFT_RIGHT),
    ('7', &PIPE_DOWN_LEFT),
];

static OPPOSITES: [(&Direction, &Direction); 4] =
    [(&UP, &DOWN), (&DOWN, &UP), (&LEFT, &RIGHT), (&RIGHT, &LEFT)];

static DIRECTIONS: [&Direction; 4] = [&UP, &DOWN, &LEFT, &RIGHT];

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    i: usize,
    j: usize,
}

struct Map {
    map: Vec<Vec<Option<&'static Pipe>>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn move_point(&self, point: &Point, direction: &Direction) -> Option<Point> {
        let i = point.i as isize + direction.del_i;
        if i < 0 || i >= self.rows as isize {
            return None;
        }
        let j = point.j as isize + direction.del_j;
        if j < 0 || j >= self.cols as isize {
            return None;
        }

        Some(Point {
            i: i as usize,
            j: j as usize,
        })
    }

    fn get_pipe(&self, point: &Point) -> Option<&Pipe> {
        if let Some(row) = self.map.get(point.i) {
            if let Some(element) = row.get(point.j) {
                return *element;
            }
        }
        None
    }
}

fn parse_input(input: &str) -> (Map, Point) {
    let symbol_map = HashMap::from(SYMBOLS);
    let mut start_point = Point { i: 0, j: 0 };
    let mut map: Vec<Vec<Option<&Pipe>>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<Option<&Pipe>> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match symbol_map.get(&c) {
                Some(pipe) => row.push(Some(pipe)),
                None => row.push(None),
            }
            if c == 'S' {
                start_point = Point { i, j };
            }
        }
        map.push(row);
    }
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    (Map { map, rows, cols }, start_point)
}

fn get_first_move(map: &Map, start_point: &Point) -> Option<&'static Direction> {
    let opposite_map = HashMap::from(OPPOSITES);
    for direction in DIRECTIONS.into_iter() {
        if let Some(candidate_point) = map.move_point(&start_point, direction) {
            let moved_from = opposite_map.get(direction).unwrap();
            if let Some(next_pipe) = map.get_pipe(&candidate_point) {
                if next_pipe.0 == *moved_from || next_pipe.1 == *moved_from {
                    return Some(direction);
                }
            }
        }
    }
    None
}

fn traverse(map: &Map, start_point: Point) -> Result<usize, String> {
    // find the next point from the starting point.
    // println!("{:?}", start_point);
    let mut next_move = get_first_move(&map, &start_point).expect(" not first move found");
    // let mut current_point = map
    //     .move_point(&start_point, &next_move)
    //     .expect("llegal first move wot");
    // println!("{:?}", current_point);
    let mut current_point = start_point.clone();

    // we now have the first step we will take
    let opposite_map = HashMap::from(OPPOSITES);
    let mut path: Vec<Point> = vec![current_point.clone()];
    loop {
        // keep moving through the map
        if let Some(next_point) = map.move_point(&current_point, &next_move) {
            if next_point == start_point {
                break;
            }

            if let Some(next_pipe) = map.get_pipe(&next_point) {
                let moved_from = opposite_map.get(&next_move).unwrap();
                // we assume that one of the directions in the pipe
                // must coincide with the moved_from direction
                // this should hold in a valid map
                path.push(next_point.clone());
                next_move = if next_pipe.1 == *moved_from {
                    next_pipe.0
                } else {
                    next_pipe.1
                };
                println!("{:?}", current_point);
                println!("{:?}", next_move);
                current_point = next_point;
                // println!("{:?}", current_point);
            }
        }
    }
    // println!("path:");
    // println!("{:?}", path);
    let path_length = path.len() / 2;

    Ok(path_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example_input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let (map, start_point) = parse_input(&example_input);
        assert_eq!(start_point, Point { i: 2, j: 0 });
        let pathlen = traverse(&map, start_point).unwrap();
        assert_eq!(pathlen, 8);
    }

    #[test]
    fn test_example_2() {
        let example_input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let (map, start_point) = parse_input(&example_input);
        assert_eq!(start_point, Point { i: 1, j: 1 });
        let pathlen = traverse(&map, start_point).unwrap();
        assert_eq!(pathlen, 4);
    }
}
