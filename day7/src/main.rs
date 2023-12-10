use std::{io::Read, char::ParseCharError, fmt::Display};

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Hand([a,b,c,d,e]) = self;
        write!(f, "{a}{b}{c}{d}{e}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card(u8);


impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            c => (c + b'0') as char,
        };
        write!(f, "{c}")
    }
}

impl Hand {
    fn kind(&self) -> HandType {
        let Hand(mut hand) = self;
        if hand.iter().all_equal() {
            return HandType::FiveOfAKind;
        }
        hand.sort_unstable();
        let [a, b, c, d, e] = hand;
        if [a,b,c,d].iter().all_equal() || [b,c,d,e].iter().all_equal() {
            return HandType::FourOfAKind;
        }
        if [a,b,c].iter().all_equal() && e == d || a == b && [c,d,e].iter().all_equal() {
            return HandType::FullHouse;
        }
        let mut kinds = hand.iter().map(|c| hand.iter().filter(|&k| k == c).count());
        if kinds.any(|n| n == 3) {
            return HandType::ThreeOfAKind;
        }
        match hand
            .iter()
            .unique()
            .map(|c| hand.iter().filter(|&k| k == c).count())
            .filter(|&n| n == 2)
            .count()
        {
            2 => HandType::TwoPair,
            1 => HandType::OnePair,
            0 => HandType::HighCard,
            n => unreachable!("We should not have more than two pairs in a hand: {n}"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind().cmp(&other.kind()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<char> for Card {
    type Error = ParseCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value : u8  = match value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            n @ '1'..='9' => n as u8 - b'0',
            _ => panic!("Could not parse {value}")
        };
        Ok(Self(value))
    }
}

fn parse(input: &str) -> Vec<(Hand, u32)> {
    input.lines().map(|line| {
        let (hand, bid) = line.split_at(5);
        let hand : [Card; 5] = hand.chars().map(|c| {
            Card::try_from(c).unwrap()
        }).collect::<Vec<Card>>().try_into().unwrap();

        let hand = Hand(hand);
        let (_, bid)= bid.split_at(1);
        let bid : u32 = bid.parse().unwrap();
        (hand, bid)
    }).collect()
}

fn solve1(input: &str) -> u32 {
    let mut hands = parse(input);
    hands.sort_unstable_by_key(|(hand, _)| *hand);
    hands.iter().enumerate().map(|(i, (_, bid))| (i as u32 + 1)*bid).sum()
}

fn solve2(input: &str) -> u32 { todo!()}

const INPUT : &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn part1() {
    assert_eq!(solve1(INPUT), 6440);
}
#[test]
fn part2() {
    assert_eq!(solve2(INPUT), 71503);
}
