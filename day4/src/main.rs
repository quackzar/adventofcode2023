use std::io::Read;

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
    let solution = solve(&input);
    println!("Solution {solution}");
}

fn solve(input: &str) -> u32 {
    let mut scores = Vec::new();
    for line in input.lines() {
        let (card, numbers) = line.split_once(':').unwrap();
        let (_, _num) = card.split_once(' ').unwrap();
        let (winning, holding) = numbers.split_once('|').unwrap();
        let winning : Vec<u8> = winning.split(' ').filter_map(|num| num.parse().ok()).collect();
        let holding : Vec<u8> = holding.split(' ').filter_map(|num| num.parse().ok()).collect();

        let mut wins = 0;
        for &win_num in &winning {
            let win = holding.iter().any(|&n| n == win_num);
            wins += win as u32;
        }
        let score = if wins != 0 {1 << (wins - 1)} else {0};
        scores.push(score);
    }
    scores.iter().sum()
}


#[test]
fn part1() {
    assert_eq!(solve(INPUT), 13);
}
