use std::{io::Read, mem::swap};

const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 1
";

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}

fn parse(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    input.lines().map(|line| {
        let (card, numbers) = line.split_once(':').unwrap();
        let (_, _num) = card.split_once(' ').unwrap();
        let (winning, holding) = numbers.split_once('|').unwrap();
        let winning : Vec<u8> = winning.split(' ').filter_map(|num| num.parse().ok()).collect();
        let holding : Vec<u8> = holding.split(' ').filter_map(|num| num.parse().ok()).collect();
        (winning, holding)
    }).collect()
}

fn score(winning: &[u8], holding: &[u8]) -> u32 {
    let mut wins = 0;
    for &win_num in winning {
        let win = holding.iter().any(|&n| n == win_num);
        wins += win as u32;
    }
    wins
}

fn solve1(input: &str) -> u32 {
    let mut total_points = Vec::new();
    let cards = parse(input);

    for (winning, holding) in cards {
        let points = score(&winning, &holding);
        let points = if points != 0 {1 << (points - 1)} else {0};
        total_points.push(points);
    }
    total_points.iter().sum()
}

fn solve2(input: &str) -> usize {
    let cards = parse(input);
    let mut card_stack1 : Vec<usize> = (0..cards.len()).collect();
    let mut card_stack2 : Vec<usize> = Vec::new();
    let mut cards_total = card_stack1.len();
    let old_cards = &mut card_stack1;
    let new_cards = &mut card_stack2;


    let scores : Vec<_> = cards.iter().map(|(winning, holding)| score(winning, holding)).collect();
    loop {
        for &i in old_cards.iter() {
            let points = scores[i];
            for j in 1..=points {
                new_cards.push(i + j as usize)
            }
        }
        if new_cards.is_empty() {
            break;
        }
        cards_total += new_cards.len();
        old_cards.clear();
        swap(old_cards, new_cards);
    };
    cards_total
}


#[test]
fn part1() {
    assert_eq!(solve1(INPUT), 13);
}
#[test]
fn part2() {
    assert_eq!(solve2(INPUT), 30);
}
