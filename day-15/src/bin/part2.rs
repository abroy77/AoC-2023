use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let seqs = parse_input(&contents);
    let solution = get_solution(&seqs);
    println!("Solution: {:?}", solution);
}
fn label_2_box(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (17 * (acc + c as usize)) % 256)
}

fn get_solution(sequences: &Vec<&str>) -> usize {
    let mut boxes: Vec<Vec<&str>> = vec![vec![]; 256];
    let mut focal_lenses: HashMap<&str, usize> = HashMap::new();
    for s in sequences {
        if s.contains('-') {
            // remove element
            if let Some((label, _)) = s.split_once('-') {
                let box_id = label_2_box(label);
                let lens_pos = boxes[box_id].iter().position(|&x| x == label);
                if let Some(pos) = lens_pos {
                    boxes[box_id].remove(pos);
                }
            }
        } else {
            // add element
            if let Some((label, focal_length)) = s.split_once('=') {
                let box_id = label_2_box(label);
                let focal_length = focal_length.parse::<usize>().unwrap();
                match boxes[box_id].iter().position(|&x| x == label) {
                    Some(pos) => {
                        // remove and update
                        boxes[box_id][pos] = label;
                        focal_lenses.insert(label, focal_length);
                    }
                    None => {
                        // add
                        boxes[box_id].push(label);
                        focal_lenses.insert(label, focal_length);
                    }
                }
            }
        }
    }

    boxes.iter().enumerate().fold(0, |acc, (box_id, contents)| {
        acc + contents
            .iter()
            .enumerate()
            .fold(0, |acc_inner, (lens_pos, label)| {
                acc_inner + (1 + box_id) * (1 + lens_pos) * focal_lenses[label]
            })
    })
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        assert_eq!(label_2_box("HASH"), 52);
    }

    #[test]
    fn test_seq() {
        let seqs = parse_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(get_solution(&seqs), 145);
    }
}
