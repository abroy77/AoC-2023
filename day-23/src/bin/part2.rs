use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let grid = parse_input(&contents);
    let solution = get_solution(grid);
    println!("Solution: {:?}", solution);
}

type Grid = Vec<Vec<char>>;

type ContractedGraph = HashMap<Point, HashMap<Point, usize>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    i: isize,
    j: isize,
}

fn get_start_end_points(grid: &Grid) -> (Point, Point) {
    let start_col = grid[0].iter().position(|&c| c == '.').unwrap();
    let num_rows = grid.len();
    let end_col = grid[num_rows - 1].iter().position(|&c| c == '.').unwrap();

    let start = Point {
        i: 0,
        j: start_col as isize,
    };
    let end = Point {
        i: (num_rows - 1) as isize,
        j: end_col as isize,
    };
    (start, end)
}

fn get_important_points(grid: &Grid) -> Vec<Point> {
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let (start, end) = get_start_end_points(grid);

    // find points which have more than 2 neighbours or are start / end points
    let mut points = vec![start, end];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                continue;
            }
            let mut neighbours = 0;
            for (di, dj) in dirs.iter() {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0
                    || nj < 0
                    || ni >= num_rows as isize
                    || nj >= num_cols as isize
                    || grid[ni as usize][nj as usize] == '#'
                {
                    continue;
                }
                neighbours += 1;
            }
            if neighbours > 2 {
                points.push(Point {
                    i: i as isize,
                    j: j as isize,
                });
            }
        }
    }
    points
}

fn edge_contract(grid: &Grid) -> ContractedGraph {
    let points = get_important_points(grid);

    // now we need a hashmap for each point, where each value is a hashmap that where the key is another point and the value is the distance to that point
    // use dfs to find adjacent nodes and distance to them
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut point_distances: HashMap<Point, HashMap<Point, usize>> = HashMap::new();
    for source_point in points.iter() {
        let mut stack = vec![(source_point.clone(), 0)];
        let mut seen = HashSet::from([source_point.clone()]);

        while let Some((target_point, distance)) = stack.pop() {
            // check if it's a point of interest
            if distance != 0 && points.contains(&target_point) {
                // add to hashmap
                let distances = point_distances
                    .entry(*source_point)
                    .or_insert(HashMap::new());
                distances.insert(target_point, distance);
                continue;
            }

            // add neighbours to stack
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            for (di, dj) in dirs.iter() {
                let ni = target_point.i + di;
                let nj = target_point.j + dj;
                let neighbour_point = Point { i: ni, j: nj };
                if ni < 0
                    || nj < 0
                    || ni >= num_rows as isize
                    || nj >= num_cols as isize
                    || seen.contains(&neighbour_point)
                    || grid[ni as usize][nj as usize] == '#'
                {
                    continue;
                }
                seen.insert(neighbour_point);
                stack.push((neighbour_point, distance + 1));
            }
        }
    }

    // only connecting adjacent nodes. ie nodes that can be reached directly from a node, without an intermediary node
    point_distances
}

fn dfs(
    graph: &ContractedGraph,
    point: &Point,
    end_point: &Point,
    seen: &mut HashSet<Point>,
) -> isize {
    if point == end_point {
        return 0;
    }

    let mut m = f64::NEG_INFINITY;
    seen.insert(point.clone());

    for (next_point, distance) in graph[point].iter() {
        if seen.contains(next_point) {
            continue;
        }
        m = m.max(dfs(graph, next_point, end_point, seen) as f64 + *distance as f64);
    }

    seen.remove(point);
    m as isize
}

fn get_solution(grid: Grid) -> isize {
    let graph = edge_contract(&grid);
    let (start, end) = get_start_end_points(&grid);
    let mut seen = HashSet::new();
    dfs(&graph, &start, &end, &mut seen)
}

fn parse_input(input: &str) -> Grid {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let grid = parse_input(input);
        let sol = get_solution(grid);
        assert_eq!(sol, 154);
    }
}
