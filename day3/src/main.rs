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

fn solve(input: &str) -> u32 {
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
    for sym in syms {
        let ((row, col), sym) = sym;
        // horizontal & diagonal
        let horizontal = nums.iter().filter(|(h, v, _)| {
            // let start = s.saturating_sub(1);
            // let end = e + 1;
            // (row as isize - *r as isize).abs() <= 1 && (col >= start && col <= end)
            h.contains(&row) && v.contains(&col)
        });

        let mut found: Vec<_> = horizontal
            .map(|(_, _, num)| Part {
                num: num.parse().unwrap(),
                sym,
            })
            .collect();

        all.append(&mut found);
    }

    let sum : u32 = all.iter().map(|p| p.num).sum();
    dbg!(sum);
    sum
}

#[test]
fn demo() {
    assert_eq!(solve(INPUT), 4361);
}
