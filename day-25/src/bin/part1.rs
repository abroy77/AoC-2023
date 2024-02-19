use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, graph) = parse_input(&contents).unwrap();
    let solution = get_solution(&graph);
    println!("Solution: {:?}", solution);
}
type Graph = HashMap<String, HashSet<String>>;
type NodeSet = HashSet<String>;

fn parse_input(input: &str) -> IResult<&str, Graph> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    let mut graph = Graph::new();
    for (key, values) in lines {
        // update graph with key and values
        // if key exists, append values to the set
        // if key does not exist, create a new set with values
        let value_set = values
            .iter()
            .cloned()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        graph
            .entry(key.to_string())
            .and_modify(|s| s.extend(value_set.clone()))
            .or_insert(value_set.clone());

        // update based on the values
        for v in value_set.iter() {
            graph
                .entry(v.clone().to_string())
                .and_modify(|s| {
                    s.insert(key.to_string());
                })
                .or_insert_with(|| {
                    let mut s = HashSet::new();
                    s.insert(key.to_string());
                    s
                });
        }
    }
    Ok((input, graph))
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1))(input)
}

fn foreign_count(graph: &Graph, set: &NodeSet, node: &String) -> usize {
    // get the length of the set subtraction of set from graph[node]

    graph.get(node).unwrap().difference(set).count()
}

fn get_solution(graph: &Graph) -> usize {
    let graphkeys = graph.keys().cloned().collect::<HashSet<_>>();
    let mut set = graphkeys.clone();

    loop {
        let foreign_counts = set
            .iter()
            .map(|node| foreign_count(graph, &set, node))
            .sum::<usize>();
        if foreign_counts == 3 {
            break;
        }

        // get the first node which has more than 1 foreign connection
        // or raise error
        let nodeshift = set
            .iter()
            .max_by_key(|node| foreign_count(graph, &set, node))
            .or_else(|| panic!(" no node found"))
            .unwrap()
            .to_owned();

        // remove node from set. implicitly it's in the other set
        set.remove(&nodeshift);
    }

    set.len() * graphkeys.difference(&set).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> &'static str {
        "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
    }

    #[test]
    fn test_solution() {
        let data = data();
        let (_, graph) = parse_input(data).unwrap();
        let solution = get_solution(&graph);
        assert_eq!(solution, 54);
    }

    #[test]
    fn test_counter() {
        let data = data();
        let (_, graph) = parse_input(data).unwrap();
        let mut set = graph.keys().cloned().collect::<HashSet<_>>();
        set.remove("xhk");
        set.remove("rhn");
        set.remove("jqt");

        let mut foreign_counts = 0;
        for node in set.iter() {
            let fc = foreign_count(&graph, &set, node);
            foreign_counts += fc;
        }

        assert_eq!(foreign_counts, 7);
    }
}
