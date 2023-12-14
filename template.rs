use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}

fn solve1(input: &str) -> u32{
    todo!()
}

fn solve2(input: &str) -> u32 {
    todo!()
}


const INPUT: &str = "\
TEST INPUT GOES HERE
";

#[test]
fn part1() {
    assert_eq!(solve1(INPUT), 0);
}

#[test]
fn part2() {
    assert_eq!(solve2(INPUT), 0);
}
