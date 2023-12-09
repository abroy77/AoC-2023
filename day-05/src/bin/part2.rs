use rayon::prelude::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need argument to filepath")
    }
    let data = fs::read_to_string(&args[1]).expect("file not present");

    let solution = get_solution(data);

    println!("Result is {solution}")
}
type Seeds = Vec<usize>;

fn parse_input(data: &str) -> (Seeds, MapSeries) {
    // first get the seeds
    // use functional programming to get the seeds
    let seeds = data
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .into_iter()
        .map(|num_str| num_str.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // we need to think of these as ranges
    //iterate through 2 a time
    let seed_ranges: Vec<(usize, usize)> =
        seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    let seeds = seed_ranges
        .iter()
        .flat_map(|(start, length)| (*start..*start + *length).collect::<Vec<usize>>())
        .collect();

    //skip the first line

    //skip the first line and keep the rest as is
    let map_series_data = data.lines().skip(3).collect::<Vec<&str>>().join("\n");
    let maps = map_series_data
        .split("map:\n")
        .map(|map_strs| {
            let mut map_range_vec = Vec::new();
            for map_str in map_strs.lines() {
                // if starts with any letters, break
                if map_str.is_empty() {
                    break;
                }
                // parse the map range
                let map_range = map_str
                    .split(" ")
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let map_range = MapRange::new(map_range[0], map_range[1], map_range[2]);
                map_range_vec.push(map_range);
            }
            Map::new(map_range_vec)
        })
        .collect::<Vec<Map>>();
    let map_series = MapSeries::new(maps);
    (seeds, map_series)
}

fn get_solution(data: String) -> usize {
    let (seeds, map_series) = parse_input(&data);
    let locations = seeds
        .par_iter()
        .map(|seed| map_series.map(*seed))
        .collect::<Vec<usize>>();

    *locations.iter().min().unwrap()
}

// let's store a map_range as a struct:
#[derive(Debug)]
struct MapRange {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl MapRange {
    fn new(dest_start: usize, source_start: usize, length: usize) -> Self {
        Self {
            dest_start,
            source_start,
            length,
        }
    }

    fn in_range(&self, source_index: &usize) -> bool {
        *source_index >= self.source_start && *source_index < self.source_start + self.length
    }

    fn map(&self, source_index: usize) -> usize {
        let distance = source_index - self.source_start;
        self.dest_start + distance
    }
}

// make a map a collection of MapRanges
#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn new(ranges: Vec<MapRange>) -> Self {
        Self { ranges }
    }

    fn map(&self, source_index: usize) -> usize {
        match self
            .ranges
            .iter()
            .find(|range| range.in_range(&source_index))
        {
            Some(range) => range.map(source_index),
            None => source_index,
        }
    }
}

// make a MapSeries a sequence of Maps
#[derive(Debug)]
struct MapSeries {
    maps: Vec<Map>,
}

impl MapSeries {
    fn new(maps: Vec<Map>) -> Self {
        Self { maps }
    }

    fn map(&self, source_index: usize) -> usize {
        self.maps
            .iter()
            .fold(source_index, |index, map| map.map(index))

        // let mut index = source_index;
        // for map in &self.maps {
        //     index = map.map(index);
        // }
        // index
    }
}
#[cfg(test)]
mod tests {}
