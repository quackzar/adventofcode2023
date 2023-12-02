const INPUT : &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

use regex::Regex;

fn parse(game: &str) -> Vec<Vec<(i32, Color)>> {
    let re = Regex::new(r"(?<num>\d+) (?<color>\w+)").unwrap();
    // remove the first part
    let game = game.split(": ").last().unwrap();
    game.split(';').map(|s| {
        s.split(',')
            .map(|s| {
            let caps = re.captures(s).unwrap();
            let num = &caps["num"];
            let color = &caps["color"];
            let num : i32 = num.parse().unwrap();
            let color = match color {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                _ => panic!("could not handle: {color}"),
            };
            (num, color)
        }).collect::<Vec<_>>()
    }).collect()
}



fn main() {
    let red_power = 12;
    let green_power = 13;
    let blue_power = 14;

    let re = Regex::new(r"^Game (?<id>\d+)").unwrap();
    let mut good_games = Vec::new();
    let mut powers = Vec::new();
    for line in INPUT.lines() {
        let Some(caps) = re.captures(line) else {
            continue;
        };
        let id : u32 = caps["id"].parse().unwrap();

        let game = parse(line);
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;
        for draw in game {
            for balls in draw {
                match balls {
                    (num, Color::Red) => reds = reds.max(num),
                    (num, Color::Green) => greens = greens.max(num),
                    (num, Color::Blue) => blues = blues.max(num),
                }
            }
        }
        if reds <= red_power && greens <= green_power && blues <= blue_power {
            good_games.push(id);
        }
        let power = reds * greens * blues;
        powers.push(power)
    }

    println!("Part 1");
    println!("Good games: {good_games:#?}");
    let sum : u32 = good_games.into_iter().sum();
    println!("Sum = {sum}\n");

    println!("Part 2");
    println!("Powers: {powers:#?}");
    let sum : i32 = powers.into_iter().sum();
    println!("Sum = {sum}");
}
