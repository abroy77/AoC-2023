use std::env;
use std::fs;
#[derive(Debug)]
struct Schematic {
    digits: Vec<Vec<Option<usize>>>,
    symbols: Vec<Vec<bool>>,
    dimensions: (usize, usize), // (row,column)
}
// we are using (i,j) coordinates ie row, column coordinates
type Point = (usize, usize);

const NOT_SYMBOLS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

impl Schematic {
    fn from_string(schematic_str: &str) -> Schematic {
        let mut digits: Vec<Vec<Option<usize>>> = Vec::new();
        let mut symbols: Vec<Vec<bool>> = Vec::new();
        for line in schematic_str.lines() {
            let mut digits_row: Vec<Option<usize>> = Vec::new();
            let mut symbols_row: Vec<bool> = Vec::new();
            for c in line.chars() {
                // push digit or None to numbers
                match c.to_digit(10) {
                    Some(digit) => digits_row.push(Some(digit as usize)),
                    None => digits_row.push(None),
                }
                if !NOT_SYMBOLS.contains(&c) {
                    symbols_row.push(true);
                } else {
                    symbols_row.push(false);
                }
            }
            digits.push(digits_row);
            symbols.push(symbols_row);
        }
        let num_lines = schematic_str.lines().count();
        let line_width = schematic_str.lines().next().unwrap().len();
        let dimensions = (num_lines, line_width);
        Schematic {
            digits,
            symbols,
            dimensions,
        }
    }

    fn get_number(&self, point: Point) -> Option<usize> {
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

    fn is_neighbour_symbol(&self, point: Point, num_length: usize) -> bool {
        // get the coordinates for all the neighbours

        point.0.checked_sub(1).unwrap_or(0);
        let mut neighbours: Vec<Point> = vec![
            (
                point.0.checked_sub(1).unwrap_or(point.0),
                point.1.checked_sub(1).unwrap_or(point.1),
            ), // top left
            (point.0, point.1.checked_sub(1).unwrap_or(point.1)), // left
            (point.0 + 1, point.1.checked_sub(1).unwrap_or(point.1)), // bottom left
            (
                point.0.checked_sub(1).unwrap_or(point.0),
                point.1 + num_length,
            ), // top right
            (point.0, point.1 + num_length),                      // right
            (point.0 + 1, point.1 + num_length),                  // bottom right
                                                                  // we don't need to worry about wrapping through the top and left
                                                                  // beacuse we'll wrap to such large indexes that the get method
                                                                  // will return None
        ];
        // add top and bottom points through the word
        for j in 0..num_length {
            neighbours.push((point.0.checked_sub(1).unwrap_or(point.0), point.1 + j));
            neighbours.push((point.0 + 1, point.1 + j));
        }

        for neighbour in neighbours {
            if let Some(row) = self.symbols.get(neighbour.0) {
                if let Some(symbol) = row.get(neighbour.1) {
                    if *symbol {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn num_digits(number: usize) -> usize {
    number.checked_ilog10().unwrap_or(0) as usize + 1
}

fn get_solution(schematic: &Schematic) -> u32 {
    let mut solution: u32 = 0;

    for i in 0..schematic.dimensions.0 {
        //iterate rows
        let mut j = 0;
        while j < schematic.dimensions.1 {
            //iterate columns
            match schematic.get_number((i, j)) {
                Some(number) => {
                    let num_length = num_digits(number);
                    if schematic.is_neighbour_symbol((i, j), num_length) {
                        solution += number as u32;
                    }
                    j += num_length;
                }

                None => {
                    j += 1;
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
    use super::num_digits;
    use super::Point;
    use super::Schematic;
    use super::NOT_SYMBOLS;

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
        let number = test_data.get_number(point);
        assert_eq!(number, Some(114));
    }

    #[test]
    fn test_get_number_58() {
        let test_data = make_test_data();
        let point: Point = (5, 7);
        let number = test_data.get_number(point);
        assert_eq!(number, Some(58));
    }

    #[test]
    fn num_digits_test() {
        assert_eq!(num_digits(114), 3);
        assert_eq!(num_digits(58), 2);
        assert_eq!(num_digits(3475), 4);
    }

    #[test]
    fn neighbour_35() {
        let test_data = make_test_data();
        let point: Point = (2, 2);
        let num_length = num_digits(35);
        let is_neighbour = test_data.is_neighbour_symbol(point, num_length);
        assert_eq!(is_neighbour, true);
    }

    #[test]
    fn not_neighbour_114() {
        let test_data = make_test_data();
        let point: Point = (0, 5);
        let num_length = num_digits(114);
        let is_neighbour = test_data.is_neighbour_symbol(point, num_length);
        assert_eq!(is_neighbour, false);
    }
    #[test]
    fn solution_test() {
        let test_data = make_test_data();
        let solution = get_solution(&test_data);
        assert_eq!(solution, 4361);
    }
}
