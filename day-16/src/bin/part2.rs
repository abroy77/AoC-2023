use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let grid = Grid::new(&contents);
    let start_states = grid.possible_starts();
    let solution = start_states
        .into_iter()
        .map(|s| get_solution(&grid, s))
        .max()
        .unwrap();
    println!("Solution: {:?}", solution);
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    i: i32,
    j: i32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Movement {
    del_i: i32,
    del_j: i32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Part {
    MirrorBack,      //backslash
    MirrorFront,     // forward slash
    SplitHorizontal, // |
    SplitVertical,   // -
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    point: Point,
    movement: Movement,
}

struct Grid {
    grid: Vec<Vec<Option<Part>>>,
}
impl Grid {
    fn new(input: &str) -> Self {
        let mut rows = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let part = match c {
                    '/' => Some(Part::MirrorBack),
                    '\\' => Some(Part::MirrorFront),
                    '|' => Some(Part::SplitHorizontal),
                    '-' => Some(Part::SplitVertical),
                    _ => None,
                };
                row.push(part);
            }
            rows.push(row);
        }
        Grid { grid: rows }
    }
    fn is_valid_state(&self, state: &State) -> bool {
        let i = state.point.i;
        let j = state.point.j;
        if i < 0 || j < 0 {
            return false;
        }
        let num_rows = self.grid.len();
        if i as usize >= num_rows {
            return false;
        }
        let num_cols = self.grid[i as usize].len();
        if j as usize >= num_cols {
            return false;
        }
        true
    }
    fn next_state(&self, state: State) -> Vec<State> {
        let part = &self.grid[state.point.i as usize][state.point.j as usize];
        match part {
            Some(part) => part
                .next_state(&state)
                .into_iter()
                .filter(|s| self.is_valid_state(s))
                .collect::<Vec<State>>(),
            None => {
                let next_point = Point {
                    i: state.point.i + state.movement.del_i,
                    j: state.point.j + state.movement.del_j,
                };
                let next_state = State {
                    point: next_point,
                    movement: state.movement,
                };
                if self.is_valid_state(&next_state) {
                    vec![next_state]
                } else {
                    vec![]
                }
            }
        }
    }
    fn possible_starts(&self) -> Vec<State> {
        let mut start_states = Vec::new();
        // top row and bottom row
        for j in 0..self.grid[0].len() {
            for (i, del_i) in [0, self.grid.len() - 1].iter().zip([1, -1].iter()) {
                let start_state = State {
                    point: Point {
                        i: *i as i32,
                        j: j as i32,
                    },
                    movement: Movement {
                        del_i: *del_i,
                        del_j: 0,
                    },
                };
                start_states.push(start_state);
            }
        }
        // left column and right column
        for i in 0..self.grid.len() {
            for (j, del_j) in [0, self.grid[0].len() - 1].iter().zip([1, -1].iter()) {
                let start_state = State {
                    point: Point {
                        i: i as i32,
                        j: *j as i32,
                    },
                    movement: Movement {
                        del_i: 0,
                        del_j: *del_j,
                    },
                };
                start_states.push(start_state);
            }
        }

        start_states
    }
}
impl Part {
    fn next_state(&self, state: &State) -> Vec<State> {
        match self {
            Part::MirrorBack => {
                let new_direction = Movement {
                    del_i: -state.movement.del_j,
                    del_j: -state.movement.del_i,
                };
                let new_point = Point {
                    i: state.point.i + new_direction.del_i,
                    j: state.point.j + new_direction.del_j,
                };
                vec![State {
                    point: new_point,
                    movement: new_direction,
                }]
            }
            Part::MirrorFront => {
                let new_direction = Movement {
                    del_i: state.movement.del_j,
                    del_j: state.movement.del_i,
                };
                let new_point = Point {
                    i: state.point.i + new_direction.del_i,
                    j: state.point.j + new_direction.del_j,
                };
                vec![State {
                    point: new_point,
                    movement: new_direction,
                }]
            }
            Part::SplitHorizontal => {
                if state.movement.del_j != 0 {
                    //split when light moving horizontally
                    let mut new_states = Vec::new();
                    for del_i in [-1, 1] {
                        let new_direction = Movement {
                            del_i: del_i,
                            del_j: 0,
                        };
                        let new_point = Point {
                            i: state.point.i + new_direction.del_i,
                            j: state.point.j + new_direction.del_j,
                        };
                        new_states.push(State {
                            point: new_point,
                            movement: new_direction,
                        });
                    }
                    new_states
                } else {
                    self.keep_going(&state)
                }
            }
            Part::SplitVertical => {
                if state.movement.del_i != 0 {
                    //split when light moving vertically
                    let mut new_states = Vec::new();
                    for del_j in [-1, 1] {
                        let new_direction = Movement {
                            del_i: 0,
                            del_j: del_j,
                        };
                        let new_point = Point {
                            i: state.point.i + new_direction.del_i,
                            j: state.point.j + new_direction.del_j,
                        };
                        new_states.push(State {
                            point: new_point,
                            movement: new_direction,
                        });
                    }
                    new_states
                } else {
                    self.keep_going(&state)
                }
            }
        }
    }

    fn keep_going(&self, state: &State) -> Vec<State> {
        let new_point = Point {
            i: state.point.i + state.movement.del_i,
            j: state.point.j + state.movement.del_j,
        };
        vec![State {
            point: new_point,
            movement: state.movement,
        }]
    }
}

fn get_solution(grid: &Grid, start_state: State) -> usize {
    let mut seen_points: HashSet<Point> = HashSet::new();
    let mut seen_states: HashSet<State> = HashSet::new();
    let mut queue: VecDeque<State> = VecDeque::new();
    let start_point = Point { i: 0, j: 0 };
    let start_movement = Movement { del_i: 0, del_j: 1 };
    queue.push_back(start_state);

    while let Some(state) = queue.pop_front() {
        if let false = seen_states.insert(state.clone()) {
            continue;
        }
        seen_points.insert(state.point);

        seen_states.insert(state.clone());
        let next_states = grid.next_state(state);
        for next_state in next_states {
            queue.push_back(next_state);
        }
    }
    seen_points.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex() {
        let data = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        let grid = Grid::new(data);
        let start_state = State {
            point: Point { i: 0, j: 0 },
            movement: Movement { del_i: 0, del_j: 1 },
        };
        assert_eq!(get_solution(&grid, start_state), 46);
    }
}
