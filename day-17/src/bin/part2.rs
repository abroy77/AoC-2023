use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let grid = Grid::new(&contents);
    let solution = get_solution(&grid);
    println!("Solution: {:?}", solution);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    point: (i32, i32),
    movement: (i32, i32),
    straight_steps: usize,
    cost: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
struct Grid {
    grid: Vec<Vec<usize>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut rows = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as usize);
            }
            rows.push(row);
        }
        Grid { grid: rows }
    }
    fn point_in_grid(&self, point: (i32, i32)) -> bool {
        if point.0 < 0
            || point.1 < 0
            || point.0 >= self.grid.len() as i32
            || point.1 >= self.grid[0].len() as i32
        {
            return false;
        }
        true
    }

    fn neighbour_states(&self, state: &State) -> Vec<State> {
        // can only turn right, left, or keep going
        // back is the negative of the current movement
        let mut movements = Vec::new();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        if state.movement == (0, 0) {
            movements.extend(vec![(0, 1), (1, 0)]);
        } else if state.straight_steps < 4 {
            movements.push(state.movement);
        } else if state.straight_steps >= 4 && state.straight_steps < 10 {
            for direction in directions {
                if direction != (-state.movement.0, -state.movement.1) {
                    movements.push(direction);
                }
            }
        } else if state.straight_steps >= 10 {
            for direction in directions {
                if direction != (-state.movement.0, -state.movement.1)
                    && direction != (state.movement.0, state.movement.1)
                {
                    movements.push(direction);
                }
            }
        }

        // make new states
        let mut new_states = Vec::new();
        for movement in movements {
            let new_point = (state.point.0 + movement.0, state.point.1 + movement.1);
            if !self.point_in_grid(new_point) {
                continue;
            }
            let new_cost = state.cost + self.grid[new_point.0 as usize][new_point.1 as usize];
            let new_straight_steps = if movement == state.movement {
                state.straight_steps + 1
            } else {
                1
            };
            let new_state = State {
                point: new_point,
                movement,
                cost: new_cost,
                straight_steps: new_straight_steps,
            };
            new_states.push(new_state);
        }

        new_states
    }
}

fn get_solution(grid: &Grid) -> Option<usize> {
    let mut seen_points: HashSet<((i32, i32), (i32, i32), usize)> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut debug_grid: Vec<Vec<usize>> = vec![vec![0; grid.grid[0].len()]; grid.grid.len()];
    let start_state = State {
        point: (0, 0),
        movement: (0, 0),
        cost: 0,
        straight_steps: 0,
    };
    queue.push(start_state);

    while let Some(state) = queue.pop() {
        if let Some(_) = seen_points.get(&(state.point, state.movement, state.straight_steps)) {
            continue;
        }
        debug_grid[state.point.0 as usize][state.point.1 as usize] = state.cost;

        if state.point == (grid.grid.len() as i32 - 1, grid.grid[0].len() as i32 - 1)
            && state.straight_steps >= 4
        {
            return Some(state.cost);
        }

        let next_states = grid.neighbour_states(&state);
        for next_state in next_states {
            queue.push(next_state);
            seen_points.insert((state.point, state.movement, state.straight_steps));
        }
    }

    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_solution() {
        use super::*;
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let grid = Grid::new(&input);
        let solution = get_solution(&grid);
        assert_eq!(solution, Some(94));
    }
}
