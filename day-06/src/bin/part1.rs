use nom::character::complete;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
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

    let (_, (times, distance)) = parse_input(&data).unwrap();

    // zip times and distance together together as tuples
    let times_distance: Vec<(u32, u32)> = times.into_iter().zip(distance.into_iter()).collect();

    let sol = get_solution(times_distance);
    println!("Result is {}", sol);
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (_, data) = separated_list1(multispace1, complete::u32)(input)?;
    Ok((input, data))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let mut lines = input.lines();
    // lines.next().unwrap();
    let (_, times) = parse_line(lines.next().unwrap())?;
    let (_, distance) = parse_line(lines.next().unwrap())?;

    // convert to arrays
    Ok((input, (times, distance)))
}

fn get_record_range_length(total_time: &u32, record: &u32) -> u32 {
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
    let low_boundary = low_boundary.ceil() as u32;
    let high_boundary = high_boundary.ceil() as u32;
    (low_boundary..high_boundary).count() as u32
}

fn get_solution(times_distance: Vec<(u32, u32)>) -> u32 {
    times_distance.iter().fold(1, |prod, (time, distance)| {
        prod * get_record_range_length(time, distance)
    })
}
