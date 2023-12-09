use nom::character::complete;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace1},
    multi::separated_list1,
    IResult,
};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need argument to filepath")
    }
    let data = fs::read_to_string(&args[1]).expect("file not present");

    let (_, (time, distance)) = parse_input(&data).unwrap();

    // zip times and distance together together as tuples

    let sol = get_solution(vec![(time, distance)]);
    println!("Result is {}", sol);
}

fn parse_line(input: &str) -> IResult<&str, u64> {
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (_, data) = separated_list1(multispace1, digit1)(input)?;
    let full_number = data.join("").parse::<u64>().expect("not a valid number");
    Ok((input, full_number))
}

fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    let mut lines = input.lines();
    let (_, time) = parse_line(lines.next().unwrap())?;
    let (_, distance) = parse_line(lines.next().unwrap())?;

    // convert to arrays
    Ok((input, (time, distance)))
}

fn get_record_range_length(total_time: &u64, record: &u64) -> u64 {
    // get the roots of the equation
    // h = hold time = velocity
    // T = total time
    // R = record
    // h(T-h) = R ->
    // h^2 - Th + R = 0 ->
    // a = 1, b = -T, c = R
    // h = (-b +- sqrt(b^2 - 4ac)) / 2a
    // since we know h, T, R are positive
    // we can use the quadratic formula
    let low_boundary =
        (*total_time as f32 - ((total_time.pow(2) - (4 * record)) as f32).sqrt()) / (2) as f32;
    let high_boundary =
        (*total_time as f32 + ((total_time.pow(2) - (4 * record)) as f32).sqrt()) / (2) as f32;

    // for values between these ranges, the distance travelled is greater
    // than the record
    let low_boundary = low_boundary.ceil() as u64;
    let high_boundary = high_boundary.ceil() as u64;
    (low_boundary..high_boundary).count() as u64
}

fn get_solution(times_distance: Vec<(u64, u64)>) -> u64 {
    times_distance.iter().fold(1, |prod, (time, distance)| {
        prod * get_record_range_length(time, distance)
    })
}
