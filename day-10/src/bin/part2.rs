use std::collections::HashMap;
use std::env;
use std::fs;
use std::vec;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file");
    let (map, start_point) = parse_input(&input);
    println!("rows {}\ncolumns {}", map.rows, map.cols);

    let path = traverse(&map, start_point).ok().unwrap();
    // we now have the path. we need to make a bool
    // map where true are path points, false are not

    // replace start
    let first_moves = get_first_moves(&map, &start_point);
    let map = replace_start(map, &start_point, first_moves);

    let wall_counts = make_wall_counts(&map, &path);
    let solution = count_inside_points(wall_counts);

    println!("start position = {:?}", start_point);
    println!("Solution: {:?}", solution);
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

static PIPES: [&Pipe; 6] = [
    &PIPE_UP_DOWN,
    &PIPE_UP_LEFT,
    &PIPE_UP_RIGHT,
    &PIPE_LEFT_RIGHT,
    &PIPE_DOWN_LEFT,
    &PIPE_DOWN_RIGHT,
];

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

struct PathMap {
    map: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
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

fn get_first_moves(map: &Map, start_point: &Point) -> Vec<&'static Direction> {
    let opposite_map = HashMap::from(OPPOSITES);
    let mut start_directions = vec![];

    for direction in DIRECTIONS.into_iter() {
        if let Some(candidate_point) = map.move_point(&start_point, direction) {
            let moved_from = opposite_map.get(direction).unwrap();
            if let Some(next_pipe) = map.get_pipe(&candidate_point) {
                if next_pipe.0 == *moved_from || next_pipe.1 == *moved_from {
                    start_directions.push(direction);
                }
            }
        }
    }
    start_directions
}

fn traverse(map: &Map, start_point: Point) -> Result<Vec<Point>, String> {
    // find the next point from the starting point.
    // println!("{:?}", start_point);
    let next_moves = get_first_moves(&map, &start_point);
    let mut next_move = next_moves.first().unwrap().clone();
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
                // println!("{:?}", current_point);
                // println!("{:?}", next_move);
                current_point = next_point;
                // println!("{:?}", current_point);
            }
        }
    }
    // println!("path:");
    // println!("{:?}", path);

    Ok(path)
}

fn replace_start(mut map: Map, start_point: &Point, start_moves: Vec<&Direction>) -> Map {
    // figure out what kind of pipe is needed
    let mut start_pipe = None;
    for pipe in PIPES.iter() {
        if pipe.0 == start_moves[0] && pipe.1 == start_moves[1]
            || pipe.0 == start_moves[1] && pipe.1 == start_moves[0]
        {
            println!("start pipe {:?}", pipe);
            start_pipe = Some(*pipe);
        }
    }

    map.map[start_point.i][start_point.j] = start_pipe;
    println!("start point {:?}", start_point);

    map
}

fn make_wall_counts(map: &Map, path: &Vec<Point>) -> Vec<Vec<usize>> {
    let mut wall_counts = vec![vec![0; map.cols]; map.rows];
    for (i, row) in map.map.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if (i, j) == (5, 1) {
                println!("here");
            }
            // make a ray and count how many times it croses a wall
            //count true
            let mut wall_count = 0;
            if !path.contains(&Point { i, j }) && j < map.cols - 1 {
                let mut ray = j.clone() + 1;

                'ray_tracer: while ray < map.cols {
                    if let Some(pipe) = row[ray] {
                        if !path.contains(&Point { i, j: ray }) {
                            ray += 1;
                            continue;
                        }
                        if pipe == &PIPE_UP_DOWN {
                            wall_count += 1;
                            ray += 1;
                            continue;
                        } else if pipe == &PIPE_DOWN_RIGHT || pipe == &PIPE_UP_RIGHT {
                            // entered a skirt
                            // look for skirt exit
                            'skirt_tracer: loop {
                                ray += 1;
                                if ray >= map.cols {
                                    break 'ray_tracer;
                                }
                                if let Some(next_pipe) = row[ray] {
                                    if next_pipe == &PIPE_UP_LEFT || next_pipe == &PIPE_DOWN_LEFT {
                                        // found skirt exit
                                        // check if in same direction of diff directions
                                        let pipe_ud =
                                            if pipe == &PIPE_DOWN_RIGHT { &DOWN } else { &UP };

                                        let next_pipe_ud = if next_pipe == &PIPE_DOWN_LEFT {
                                            &DOWN
                                        } else {
                                            &UP
                                        };

                                        // is same direction, no change in wall count
                                        if pipe_ud != next_pipe_ud {
                                            wall_count += 1;
                                        }
                                        break 'skirt_tracer;
                                    }
                                }
                            }
                            ray += 1;
                            continue;
                        }
                    }
                    ray += 1;
                }
            }
            wall_counts[i][j] = wall_count;
        }
    }
    wall_counts
}

fn count_inside_points(wall_counts: Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for row in wall_counts.iter() {
        for element in row.iter() {
            if *element % 2 == 1 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1_test() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let (map, start_point) = parse_input(&input);
        let path = traverse(&map, start_point).ok().unwrap();
        // for row in map.map.iter() {
        //     println!("{:?}", row);
        // }
        println!("path {:?}", path);

        let first_moves = get_first_moves(&map, &start_point);
        assert_eq!(first_moves.len(), 2);
        let map = replace_start(map, &start_point, first_moves);
        let wall_counts = make_wall_counts(&map, &path);
        for row in wall_counts.iter() {
            println!("{:?}", row);
        }
        let solution = count_inside_points(wall_counts);
        assert_eq!(solution, 4);
    }

    #[test]
    fn example_2_test() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let (map, start_point) = parse_input(&input);
        let path = traverse(&map, start_point).ok().unwrap();
        for row in map.map.iter() {
            println!("{:?}", row);
        }
        let first_moves = get_first_moves(&map, &start_point);
        assert_eq!(first_moves.len(), 2);
        let map = replace_start(map, &start_point, first_moves);
        let wall_counts = make_wall_counts(&map, &path);
        for row in wall_counts.iter() {
            println!("{:?}", row);
        }
        let solution = count_inside_points(wall_counts);
        assert_eq!(solution, 8);
    }

    #[test]
    fn example_3_test() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let (map, start_point) = parse_input(&input);
        let path = traverse(&map, start_point).ok().unwrap();
        for row in map.map.iter() {
            println!("{:?}", row);
        }
        let first_moves = get_first_moves(&map, &start_point);
        assert_eq!(first_moves.len(), 2);
        let map = replace_start(map, &start_point, first_moves);
        let wall_counts = make_wall_counts(&map, &path);
        for row in wall_counts.iter() {
            println!("{:?}", row);
        }
        let solution = count_inside_points(wall_counts);
        assert_eq!(solution, 10);
    }
}
