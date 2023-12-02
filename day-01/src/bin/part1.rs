use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need argument to filepath")
    }

    let mut sum = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(ip) = line {
                sum += str_2_num(&ip);
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

fn str_2_num(input_line: &str) -> u32 {
    let mut digits = input_line.chars().filter_map(|c| c.to_digit(10));
    let first = match digits.next() {
        Some(d) => d,
        None => 0,
    };

    let last = match digits.last() {
        Some(d) => d,
        None => first,
    };

    first * 10 + last
}

// add tests
#[cfg(test)]
mod tests {
    use crate::str_2_num;

    #[test]
    fn str_2_num_1() {
        assert_eq!((str_2_num("1abc2")), 12)
    }

    #[test]
    fn str_2_num_2() {
        assert_eq!((str_2_num("pqr3stu8vwx")), 38)
    }

    #[test]
    fn str_2_num_3() {
        assert_eq!((str_2_num("a1b2c3d4e5f")), 15)
    }

    #[test]
    fn str_2_num_4() {
        assert_eq!((str_2_num("treb7uchet")), 77)
    }
    #[test]
    fn lines_test() {
        let text = "1abc2
                    pqr3stu8vwx
                    a1b2c3d4e5f
                    treb7uchet"
            .to_string();
        let mut sum = 0;
        for line in text.lines() {
            sum += str_2_num(&line);
        }
        assert_eq!(sum, 142)
    }
}
