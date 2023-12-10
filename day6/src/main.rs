use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}

fn solve1(input: &str) -> u32 {
    let races = parse(input);
    let mut prod = 1;
    for (time, dist) in races {
        // dist = speed * rem_time;
        // dist = speed * (time - time_spend)
        // dist = time_spend * (time - time_spend)
        // dist = tT - t^2;
        // dist' =  T - 2t;

        let travel = |t: u32| (time*t).saturating_sub(t.saturating_mul(t));

        let mut ways = 0;
        for i in 0..=time {
            ways += (travel(i) > dist) as u32
        }
        prod *= ways;
    }
    prod
}

fn solve2(input: &str) -> u32 {
    let (time, dist) = parse2(input);
    let travel = |t: u64| (time*t).saturating_sub(t.saturating_mul(t));
    let mut ways = 0;
    for i in 0..=time {
        ways += (travel(i) > dist) as u32
    }
    ways
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let mut input = input.lines();
    let line1 = input.next().unwrap();
    let line2 = input.next().unwrap();
    let times : Vec<u32> = re.find_iter(line1).map(|s| s.as_str().parse().unwrap()).collect();
    let distances : Vec<u32> = re.find_iter(line2).map(|s| s.as_str().parse().unwrap()).collect();
    times.into_iter().zip(distances).collect()
}

fn parse2(input: &str) -> (u64, u64) {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let mut input = input.lines();
    let line1 = input.next().unwrap();
    let line2 = input.next().unwrap();
    let times : String = re.find_iter(line1).map(|s| s.as_str()).collect();
    let distances : String = re.find_iter(line2).map(|s| s.as_str()).collect();
    (times.parse().unwrap(), distances.parse().unwrap())
}


const INPUT : &str = "\
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn part1() {
    assert_eq!(solve1(INPUT), 288);
}
#[test]
fn part2() {
    assert_eq!(solve2(INPUT), 71503);
}
