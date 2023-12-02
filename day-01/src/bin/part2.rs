use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need argument to filepath")
    }

    let digit_map = make_digit_map();

    let mut sum = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(ip) = line {
                sum += str_2_num(&ip, &digit_map);
            }
        }
    }

    println!("Result is {sum}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn make_digit_map() -> HashMap<String, u32> {
    let digit_map: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);
    digit_map
}
fn str_2_num(input_line: &str, digitmap: &HashMap<String, u32>) -> u32 {
    let mut digits: Vec<u32> = Vec::with_capacity(10);
    let line_length = input_line.len();

    for (i, c) in input_line.char_indices() {
        match c.to_digit(10) {
            Some(d) => digits.push(d),
            None => {
                // check if there's a digitmap key
                for (k, v) in digitmap.iter() {
                    // check if there's enough chars left in
                    // the line for it to be the digit string

                    let digit_str_len = k.len();
                    if i + digit_str_len <= line_length {
                        let candidate = input_line[i..i + digit_str_len].to_string();
                        if candidate == *k {
                            digits.push(*v);
                            break;
                        }
                    }
                }
            }
        }
    }

    let num_digits = digits.len();
    if num_digits > 1 {
        digits[0] * 10 + digits[num_digits - 1]
    } else if num_digits == 1 {
        digits[0] * 10 + digits[0]
    } else {
        0
    }
}

// add tests
#[cfg(test)]
mod tests {
    use crate::{make_digit_map, str_2_num};

    #[test]
    fn str_2_num_1() {
        let digit_map = make_digit_map();
        assert_eq!((str_2_num("two1nine", &digit_map)), 29)
    }

    #[test]
    fn str_2_num_2() {
        let digit_map = make_digit_map();
        assert_eq!((str_2_num("eightwothree", &digit_map)), 83)
    }

    #[test]
    fn str_2_num_3() {
        let digit_map = make_digit_map();
        assert_eq!((str_2_num("abcone2threexyz", &digit_map)), 13)
    }

    #[test]
    fn str_2_num_4() {
        let digit_map = make_digit_map();
        assert_eq!((str_2_num("xtwone3four", &digit_map)), 24)
    }

    #[test]
    fn str_2_num_5() {
        let digit_map = make_digit_map();
        assert_eq!((str_2_num("4nineeightseven2", &digit_map)), 42)
    }

    #[test]
    fn str_2_num_6() {
        let digit_map = make_digit_map();
        assert_eq!((str_2_num("zoneight234", &digit_map)), 14)
    }

    #[test]
    fn str_2_num_7() {
        let digit_map = make_digit_map();
        assert_eq!((str_2_num("7pqrstsixteen", &digit_map)), 76)
    }

    #[test]
    fn lines_test() {
        let digit_map = make_digit_map();
        let text = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen"
            .to_string();
        let mut sum = 0;
        for line in text.lines() {
            sum += str_2_num(&line, &digit_map);
        }
        assert_eq!(sum, 281)
    }
}
