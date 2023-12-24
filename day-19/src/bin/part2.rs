use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, one_of},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};
use rayon::prelude::*;
use std::fs;
use std::ops::RangeInclusive;
use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, workflows) = parse_input(&contents).unwrap();
    let solution = get_solution(&workflows);
    println!("Solution: {:?}", solution);
}
type Part = HashMap<char, RangeInclusive<usize>>;
#[derive(Debug)]
struct Condition {
    field: char,
    inequality: char,
    value: usize,
}
impl Condition {
    fn split_range(&self, part: Part) -> (Part, Part) {
        let range = part.get(&self.field).unwrap();

        let (true_range, false_range) = match self.inequality {
            '<' => (*range.start()..=self.value - 1, self.value..=*range.end()),
            '>' => (self.value + 1..=*range.end(), *range.start()..=self.value),
            _ => panic!("Invalid inequality"),
        };

        let mut true_part = part.clone();
        true_part.insert(self.field, true_range);

        let mut false_part = part.clone();
        false_part.insert(self.field, false_range);

        (true_part, false_part)
    }
}
#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    target: String,
}
impl Rule {
    fn filter_range(&self, part: Part) -> (Part, Part) {
        // true part range and false part range

        match &self.condition {
            None => (
                part.clone(),
                HashMap::from([('x', 0..=0), ('m', 0..=0), ('a', 0..=0), ('s', 0..=0)]),
            ),
            Some(cond) => cond.split_range(part),
        }
    }
}

fn get_solution(workflows: &HashMap<String, Vec<Rule>>) -> usize {
    let value_range = 1usize..=4000usize;
    let part = HashMap::from([
        ('x', value_range.clone()),
        ('m', value_range.clone()),
        ('a', value_range.clone()),
        ('s', value_range.clone()),
    ]);

    count_ways_to_pass(part, "in".to_string(), workflows)
}

fn count_ways_to_pass(
    mut ranges: Part,
    key: String,
    workflows: &HashMap<String, Vec<Rule>>,
) -> usize {
    let mut count = 0;
    if key == "R" {
        return 0;
    } else if key == "A" {
        return ranges
            .iter()
            .fold(1, |acc, (_, value)| acc * (value.end() - value.start() + 1));
    } else {
        if let Some(rules) = workflows.get(&key) {
            for rule in rules {
                let (true_range, false_range) = rule.filter_range(ranges);
                if !true_range.is_empty() {
                    count += count_ways_to_pass(true_range, rule.target.clone(), workflows);
                }
                ranges = false_range;
            }
        }
    }
    count
}

fn xmas_parser(input: &str) -> IResult<&str, char> {
    one_of("xmas")(input)
}
fn inequality_parser(input: &str) -> IResult<&str, char> {
    one_of("<>")(input)
}

fn parse_condition(input: &str) -> IResult<&str, Option<Condition>> {
    let (input, data) = tuple((xmas_parser, inequality_parser, complete::u32))(input)?;

    let cond = Condition {
        field: data.0,
        inequality: data.1,
        value: data.2 as usize,
    };

    Ok((input, Some(cond)))
}

fn parse_rule_with_condition(input: &str) -> IResult<&str, Rule> {
    let (input, (opt, target)) = separated_pair(parse_condition, tag(":"), alpha1)(input)?;
    Ok((
        input,
        Rule {
            condition: opt,
            target: target.to_string(),
        },
    ))
}

fn parse_rule_without_condition(input: &str) -> IResult<&str, Rule> {
    let (input, target) = alpha1(input)?;
    Ok((
        input,
        Rule {
            condition: None,
            target: target.to_string(),
        },
    ))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_rule_with_condition, parse_rule_without_condition))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
    let (input, key) = alpha1(input)?;

    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(complete::char(','), parse_rule),
        complete::char('}'),
    )(input)?;
    Ok((input, (key.to_string(), rules)))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<String, Vec<Rule>>> {
    let (input, data) = separated_list1(newline, parse_workflow)(input)?;
    let mut map = HashMap::new();
    for (name, rules) in data {
        map.insert(name, rules);
    }
    Ok((input, map))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, Vec<Rule>>> {
    terminated(parse_workflows, tag("\n\n"))(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        let (_, workflows) = parse_input(input).unwrap();
        let sol = get_solution(&workflows);
        assert_eq!(sol, 167409079868000)
    }
}
