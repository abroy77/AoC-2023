use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{self, alpha1, newline, one_of},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use rayon::prelude::*;
use std::fs;
use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, (workflows, parts)) = parse_input(&contents).unwrap();
    let solution = get_solution(&workflows, &parts);
    println!("Solution: {:?}", solution);
}
#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}
#[derive(Debug)]
struct Condition {
    field: char,
    inequality: char,
    value: usize,
}
#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    target: String,
}

fn get_solution(workflows: &HashMap<String, Vec<Rule>>, parts: &Vec<Part>) -> usize {
    parts
        .par_iter()
        .filter(|p| filter_part(workflows, p))
        .map(|p| p.sum())
        .sum::<usize>()
}

fn process_rules(rules: &Vec<Rule>, part: &Part) -> String {
    for rule in rules {
        match &rule.condition {
            None => return rule.target.clone(),
            Some(cond) => {
                let field = match cond.field {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Invalid field"),
                };
                match cond.inequality {
                    '<' => {
                        if field < cond.value {
                            return rule.target.clone();
                        }
                    }
                    '>' => {
                        if field > cond.value {
                            return rule.target.clone();
                        }
                    }
                    _ => panic!("Invalid inequality"),
                }
            }
        }
    }
    "R".to_string()
}

fn filter_part(workflows: &HashMap<String, Vec<Rule>>, part: &Part) -> bool {
    let mut target = "in".to_string();
    while let Some(rule) = workflows.get(&target) {
        target = process_rules(rule, part);
    }

    if target == "A" {
        true
    } else {
        false
    }
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

fn parse_part_component(input: &str) -> IResult<&str, usize> {
    let (input, _) = is_not("0123456789")(input)?;
    let (input, number) = complete::u32(input)?;
    Ok((input, number as usize))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, data) = delimited(
        tag("{"),
        separated_list1(complete::char(','), parse_part_component),
        tag("}"),
    )(input)?;

    let part = Part {
        x: data[0] as usize,
        m: data[1] as usize,
        a: data[2] as usize,
        s: data[3] as usize,
    };

    Ok((input, part))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    let (input, data) = separated_list1(tag("\n"), parse_part)(input)?;
    Ok((input, data))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<String, Vec<Rule>>, Vec<Part>)> {
    separated_pair(parse_workflows, tag("\n\n"), parse_parts)(input)
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

        let (_, (workflows, parts)) = parse_input(input).unwrap();
        let sol = get_solution(&workflows, &parts);
        assert_eq!(sol, 19114)
    }
}
