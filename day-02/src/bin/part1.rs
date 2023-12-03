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
    let max_balls: HashMap<&str, u32> = HashMap::from([("red", 12), ("blue", 14), ("green", 13)]);

    let mut sum = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(ip) = line {
                sum += str_2_num(&ip, &max_balls);
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

fn str_2_num(input_line: &str, max_balls: &HashMap<&str, u32>) -> u32 {
    let game_score_split: Vec<&str> = input_line.split(": ").collect();
    let game_id: u32 = game_score_split[0]
        .split(" ")
        .last()
        .expect(" Game ID does not exist")
        .parse()
        .expect("ID not an int");

    let score_str = game_score_split[1];

    for play in score_str.split("; ") {
        for score_set in play.split(", ") {
            let ball_count: Vec<&str> = score_set.split(" ").collect();
            let color = ball_count[1].trim();
            let num_balls: u32 = ball_count[0].parse().expect("not int count of balls");

            if num_balls > max_balls[color] {
                return 0;
            }
        }
    }
    game_id
}

// add tests
#[cfg(test)]
mod tests {
    use super::str_2_num;
    use std::collections::HashMap;

    fn make_max_balls_map() -> HashMap<&'static str, u32> {
        HashMap::from([("red", 12), ("blue", 14), ("green", 13)])
    }
    #[test]
    fn get_score_1() {
        let test_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let max_balls = make_max_balls_map();
        assert_eq!(str_2_num(&test_str, &max_balls), 1);
    }
    #[test]
    fn get_score_0() {
        let test_str = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let max_balls = make_max_balls_map();
        assert_eq!(str_2_num(&test_str, &max_balls), 0);
    }
    #[test]
    fn game_set_test() {
        let text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
        let max_balls = make_max_balls_map();
        let mut sum = 0;
        for line in text.lines() {
            sum += str_2_num(line, &max_balls);
        }
        assert_eq!(sum, 8);
    }
}
