use std::collections::HashSet;
use std::env;
use std::fs;
#[derive(Debug)]
struct Schematic {
    digits: Vec<Vec<Option<usize>>>,
    gears: Vec<Vec<bool>>,
    dimensions: (usize, usize), // (row,column)
}
// we are using (i,j) coordinates ie row, column coordinates
type Point = (usize, usize);

// const NOT_SYMBOLS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
const GEAR: char = '*';
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // top left
    (0, -1),  // left
    (1, -1),  // bottom left
    (-1, 0),  // top
    (1, 0),   // bottom
    (-1, 1),  // top right
    (0, 1),   // right
    (1, 1),   // bottom right
];

impl Schematic {
    fn from_string(schematic_str: &str) -> Schematic {
        let mut digits: Vec<Vec<Option<usize>>> = Vec::new();
        let mut gears: Vec<Vec<bool>> = Vec::new();
        for line in schematic_str.lines() {
            let mut digits_row: Vec<Option<usize>> = Vec::new();
            let mut gears_row: Vec<bool> = Vec::new();
            for c in line.chars() {
                // push digit or None to numbers
                match c.to_digit(10) {
                    Some(digit) => digits_row.push(Some(digit as usize)),
                    None => digits_row.push(None),
                }
                if GEAR == c {
                    gears_row.push(true);
                } else {
                    gears_row.push(false);
                }
            }
            digits.push(digits_row);
            gears.push(gears_row);
        }
        let num_lines = schematic_str.lines().count();
        let line_width = schematic_str.lines().next().unwrap().len();
        let dimensions = (num_lines, line_width);
        Schematic {
            digits,
            gears,
            dimensions,
        }
    }

    fn get_number(&self, point: &Point) -> Option<usize> {
        if let Some(row) = self.digits.get(point.0) {
            if let Some(Some(digit)) = row.get(point.1) {
                // we've found a digit!
                // let's check if succeeding positions also contain digits
                let mut number = digit.clone();
                let mut j = point.1 + 1;
                while let Some(Some(digit)) = row.get(j) {
                    number = number * 10 + digit;
                    j += 1;
                }
                return Some(number);
            }
        }
        None
    }

    fn get_number_start_point(&self, point: &Point) -> Option<Point> {
        if let Some(row) = self.digits.get(point.0) {
            if let Some(Some(_)) = row.get(point.1) {
                // we've found a digit!
                // let's check if preceeding positions also contain digits
                if point.1 == 0 {
                    return Some((point.0, 0));
                }

                let mut j = point.1 - 1;
                while let Some(Some(_)) = row.get(j) {
                    if j == 0 {
                        return Some((point.0, 0));
                    }
                    j -= 1;
                }
                return Some((point.0, j + 1));
            }
        }
        None
    }

    fn get_neighbour_digit_points(&self, point: &Point) -> Option<Vec<Point>> {
        // get the coordinates for all the neighbours

        let mut neighbours: Vec<Point> = Vec::new();
        for direction in DIRECTIONS.iter() {
            let new_i: isize = point.0 as isize + direction.0;
            let new_j: isize = point.1 as isize + direction.1;
            if new_i >= 0
                && new_j >= 0
                && new_i < self.dimensions.0 as isize
                && new_j < self.dimensions.1 as isize
            {
                neighbours.push((new_i as usize, new_j as usize));
            }
        }

        // add top and bottom points through the word
        let mut number_neighbour_points: Vec<Point> = Vec::new();
        for neighbour in neighbours {
            if let Some(row) = self.digits.get(neighbour.0) {
                if let Some(Some(_)) = row.get(neighbour.1) {
                    number_neighbour_points.push((neighbour.0, neighbour.1));
                }
            }
        }
        if number_neighbour_points.len() > 0 {
            return Some(number_neighbour_points);
        }
        None
    }
}

fn get_solution(schematic: &Schematic) -> u32 {
    let mut solution: u32 = 0;

    for i in 0..schematic.dimensions.0 {
        //iterate rows
        for j in 0..schematic.dimensions.1 {
            //iterate columns
            if schematic.gears[i][j] {
                // we found a gear
                // get neighbour digits
                if let Some(points) = schematic.get_neighbour_digit_points(&(i, j)) {
                    // let's get the starting point of each number that the digits are a part of
                    let start_points: Vec<Point> = points
                        .iter()
                        .filter_map(|point| schematic.get_number_start_point(point))
                        .collect::<HashSet<Point>>()
                        .into_iter()
                        .collect();

                    // check if there are exactly 2 unique starting points of neighbouring numbers
                    if start_points.len() == 2 {
                        // we have a gear with 2 neighbouring numbers!!!
                        // let's get the gear ratio
                        solution += start_points
                            .iter()
                            .filter_map(|point| schematic.get_number(point))
                            .product::<usize>() as u32;
                    }
                }
            }
        }
    }
    solution
}

fn main() {
    println!("Hello nvim bros");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need argument to filepath")
    }
    // read the file in
    let data = fs::read_to_string(&args[1]).expect("file not present");
    let schematic = Schematic::from_string(&data);
    // println!("{:?}", schematic);
    let solution = get_solution(&schematic);
    println!("Solution is {}", solution);
}

#[cfg(test)]
mod tests {
    use super::get_solution;
    use super::Point;
    use super::Schematic;

    fn make_test_data() -> Schematic {
        let test_str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
        let test_data = Schematic::from_string(&test_str);
        // println!("{:?}", test_data);
        test_data
    }

    #[test]
    fn test_get_number_114() {
        let test_data = make_test_data();
        let point: Point = (0, 5);
        let number = test_data.get_number(&point);
        assert_eq!(number, Some(114));
    }

    #[test]
    fn test_get_number_58() {
        let test_data = make_test_data();
        let point: Point = (5, 7);
        let number = test_data.get_number(&point);
        assert_eq!(number, Some(58));
    }

    #[test]
    fn get_solution_test() {
        let test_data = make_test_data();
        let solution = get_solution(&test_data);
        assert_eq!(solution, 467835);
    }
}
