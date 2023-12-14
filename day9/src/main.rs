use std::io::Read;

use itertools::Itertools;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}

fn solve1(input: &str) -> i32 {
    let mut sum = 0;
    for reading in parse(input) {
        let last_reading = extrapolate(reading);
        sum += last_reading.1;
    }
    sum
}

fn solve2(input: &str) -> i32 {
    let mut sum = 0;
    for reading in parse(input) {
        let last_reading = extrapolate(reading);
        sum += last_reading.0;
    }
    sum
}

fn extrapolate(mut reading: Vec<i32>) -> (i32, i32) {
    let mut head_and_tail : Vec<(i32, i32)> = Vec::new();
    while !reading.is_empty() {
        let last = reading.last().unwrap();
        let first = reading.first().unwrap();
        head_and_tail.push((*first, *last));
        reading = diff(&reading);
        if reading.iter().all(|&a| a == 0) {
            break;
        }
    }
    let mut diff_fst = 0;
    let mut diff_lst = 0;
    for (first, last) in head_and_tail.iter_mut().rev() {
        *last += diff_lst;
        *first -= diff_fst;
        diff_fst = *first;
        diff_lst = *last;
    }
    head_and_tail[0]
}


fn diff(list: &[i32]) -> Vec<i32> {
    let n = list.len();
    list.iter().tuple_windows().map(|(a,b)| {
        *b - *a
    }).take(n - 1).collect()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| {
        line.split(' ').map(|a| a.parse().unwrap()).collect()
    }).collect()
}

#[allow(dead_code)]
const INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[test]
fn part1() {
    assert_eq!(solve1(INPUT), 114);
}

#[test]
fn part2() {
    assert_eq!(solve2(INPUT), 2);
}
