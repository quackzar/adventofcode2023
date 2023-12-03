const INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

use std::io::Read;

#[derive(Debug)]
struct Part {
    num: u32,
    sym: char,
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    solve(&input);
}

fn solve(input: &str) -> (u32, u32) {
    let sym_re = regex::Regex::new(r"[:-@]|[!-/--.]").unwrap();
    let num_re = regex::Regex::new(r"\d+").unwrap();

    let line_length = input.find('\n').unwrap() + 1;
    let nums: Vec<_> = num_re
        .find_iter(input)
        .map(|s| {
            (
                s.start() / line_length, s.start() % line_length, s.end() % line_length, s.as_str(),
            )
        }).map(|(r,c1, c2,num)| (r.saturating_sub(1)..=r+1, c1.saturating_sub(1)..c2+1, num))
        .collect();

    let syms: Vec<_> = sym_re
        .find_iter(input)
        .map(|s| {
            (
                (s.start() / line_length, s.start() % line_length),
                s.as_str().chars().next().unwrap(),
            )
        })
        .collect();

    let mut all = Vec::new();
    let mut gear_ratios = Vec::new();
    for sym in syms {
        let ((row, col), sym) = sym;
        let mut found: Vec<_> = nums
            .iter()
            .filter(|(h, v, _)| h.contains(&row) && v.contains(&col))
            .map(|(_, _, num)| Part {
                num: num.parse().unwrap(),
                sym,
            })
            .collect();

        if sym == '*' && found.len() > 1 {
            gear_ratios.push(found.iter().map(|Part{sym: _, num}| num).product());
        }
        all.append(&mut found);
    }

    let sum : u32 = all.iter().map(|p| p.num).sum();
    let gear_sum : u32 = gear_ratios.iter().sum();
    dbg!(sum);
    dbg!(gear_sum);
    (sum, gear_sum)
}

#[test]
fn part1() {
    assert_eq!(solve(INPUT).0, 4361);
}

#[test]
fn part2() {
    assert_eq!(solve(INPUT).1, 467835);
}
