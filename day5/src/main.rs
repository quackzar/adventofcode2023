use std::{io::Read, collections::BTreeMap, u32, ops::{Range, RangeBounds}};

use itertools::{Itertools, partition};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

const INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    println!("Calculating...");
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}


fn solve1(input: &str) -> u32 {
    let mut input = input.lines();
    // Seeds
    let seeds: Vec<u32> = input
        .next()
        .unwrap()
        .split_at(7)
        .1
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();
    input.next(); // blank

    // Soil-to-seed;
    let seed2soil = parse_map(&mut input);
    let soil2fertz = parse_map(&mut input);
    let fertz2water = parse_map(&mut input);
    let water2light = parse_map(&mut input);
    let light2temp = parse_map(&mut input);
    let temp2humid = parse_map(&mut input);
    let humid2loc = parse_map(&mut input);


    let mut locations = Vec::new();

    for seed in seeds {
        let i = seed;
        let i = seed2soil.get(i);
        let i = soil2fertz.get(i);
        let i = fertz2water.get(i);
        let i = water2light.get(i);
        let i = light2temp.get(i);
        let i = temp2humid.get(i);
        let i = humid2loc.get(i);
        let loc = i;
        locations.push(loc);
    }

    let loc = locations.iter().min().unwrap();
    *loc
}

fn solve2(input: &str) -> u32 {
    let mut input = input.lines();
    // Seeds
    let seeds: Vec<Range<u32>> = input
        .next()
        .unwrap()
        .split_at(7)
        .1
        .split(' ')
        .map(|n| n.parse().unwrap())
        .tuples::<(_, _)>()
        .map(|(src, range)| (src..(src+range)))
        .collect();
    input.next(); // blank

    // Soil-to-seed;
    let seed2soil = parse_map(&mut input);
    let soil2fertz = parse_map(&mut input);
    let fertz2water = parse_map(&mut input);
    let water2light = parse_map(&mut input);
    let light2temp = parse_map(&mut input);
    let temp2humid = parse_map(&mut input);
    let humid2loc = parse_map(&mut input);


    let find_loc = |range: &Range<u32>| -> Vec<Range<u32>> {
        let res = seed2soil.many(range);
        let res: Vec<(_, _)> = res
            .iter()
            .map(|(_, pos)| pos)
            .flat_map(|pos| soil2fertz.many(pos))
            .collect();

        let res: Vec<(_, _)> = res
            .iter()
            .map(|(_, pos)| pos)
            .flat_map(|pos| fertz2water.many(pos))
            .collect();
        let res: Vec<(_, _)> = res
            .iter()
            .map(|(_, pos)| pos)
            .flat_map(|pos| water2light.many(pos))
            .collect();
        let res: Vec<(_, _)> = res
            .iter()
            .map(|(_, pos)| pos)
            .flat_map(|pos| light2temp.many(pos))
            .collect();
        let res: Vec<(_, _)> = res
            .iter()
            .map(|(_, pos)| pos)
            .flat_map(|pos| temp2humid.many(pos))
            .collect();
        let res: Vec<(_, _)> = res
            .iter()
            .map(|(_, pos)| pos)
            .flat_map(|pos| humid2loc.many(pos))
            .collect();
        res.into_iter().map(|(_, pos)| pos).collect()
    };

    dbg!(&seeds);
    let locations: Vec<Range<u32>> = seeds.iter().flat_map(find_loc).collect();
    dbg!(locations);
    let loc = locations.iter().map(|range| range.start).min().unwrap();
    loc
}



struct Almanac {
    exceptions: Vec<(u32, u32, i32)>
}

impl Almanac {
    fn new() -> Self {
        Almanac { exceptions: Vec::new() }
    }

    fn get(&self, idx: u32) -> u32 {
        for (begin, end, offset) in &self.exceptions {
            if *begin <= idx && idx < *end {
                return (offset + idx as i32) as u32
            }
        }
        idx
    }

    fn many(&self, range: &Range<u32>) -> Vec<(Range<u32>, Range<u32>)> {
        let mut new = Vec::new();
        for (rule, offset) in self.exceptions.iter().map(|(a,b,c)| (*a..*b, *c)) {
            let mapping = if rule.contains(&range.start) {
                let p1 = range.start;
                let p2 = if rule.contains(&range.end) {
                    range.end
                } else {
                    rule.end
                };
                (p1..p2, ((p1 as i32 + offset) as u32)..((p2 as i32 + offset) as u32))
            } else {
                (range.clone(), range.clone())
            };
            new.push(mapping);
        }
        new
    }


    fn insert(&mut self, begin: u32, end: u32, offset: i32) {
        self.exceptions.push((begin, end, offset));
    }
}



fn parse_map(input: &mut std::str::Lines<'_>) -> Almanac {
    input.next(); //header
    let mut map = Almanac::new();
    for line in input  {
        if line.is_empty() {
            break;
        }
        let (dst, src, range): (u32, u32, u32) = line
            .split(' ')
            .map(|n| n.parse().expect("Could not parse '{n}'"))
            .collect_tuple()
            .expect("Expected three values");

        map.insert(src, src + range, dst as i32 - src as i32);
    }
    map
}

#[test]
fn part1() {
    let solution = solve1(INPUT);
    assert_eq!(solution, 35);
}

#[test]
fn part2() {
    let solution = solve2(INPUT);
    assert_eq!(solution, 46);
}
