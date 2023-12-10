// #![allow(clippy::explicit_counter_loop)]
use std::{io::Read, collections::{BTreeMap, HashMap}, char::ParseCharError};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location([char; 3]);
impl Location {
    fn new(loc: &str) -> Self {
        let mut loc = loc.chars();
        Self([loc.next().unwrap(), loc.next().unwrap(), loc.next().unwrap()])
    }
}

enum Direction { Left, Right }
impl TryFrom<char> for Direction {
    type Error = ParseCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => panic!("Do error handling")
        }
    }
}

fn parse(input: &str) -> (Vec<Direction>, BTreeMap<Location, (Location, Location)>) {
    let mut input = input.lines();
    let map = input.next().unwrap();
    let map = map.chars().map(|c| c.try_into().unwrap()).collect();

    input.next(); // blank

    let nodes = input.map(|line| {
        let dst = Location::new(&line[0..3]);
        let left = Location::new(&line[7..10]);
        let right = Location::new(&line[12..15]);
        (dst, (left, right))

    }).collect();

    (map, nodes)
}

fn solve1(input: &str) -> u32 {
    let (map, nodes) = parse(input);
    let mut loc = Location::new("AAA");
    let mut steps = 0;
    for direction in map.iter().cycle() {
        steps += 1;
        let (left, right) = nodes[&loc];
        loc = match &direction {
            Direction::Left => left,
            Direction::Right => right,
        };
        if loc.0 == ['Z', 'Z', 'Z'] {
            break;

        }
    }
    steps
}

fn solve2(input: &str) -> u32 {
    let (map, nodes) = parse(input);
    let mut locs : Vec<_> = nodes.keys().filter(|loc| loc.0[2] == 'A').cloned().collect();
    let n = locs.len();
    let mut steps : u32 = 0;
    for direction in map.iter().cycle() {
        steps += 1;
        for i in 0..n {
            let (left, right) = nodes[&locs[i]];
            locs[i] = match &direction {
                Direction::Left => left,
                Direction::Right => right,
            };
        }
        if locs.iter().all(|c| c.0[2] == 'Z') { break; }
    }
    steps
}

const INPUT1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const INPUT2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const INPUT3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[test]
fn part1() {
    assert_eq!(solve1(INPUT1), 2);
    assert_eq!(solve1(INPUT2), 6);
}

#[test]
fn part2() {
    assert_eq!(solve2(INPUT3), 6);
}
