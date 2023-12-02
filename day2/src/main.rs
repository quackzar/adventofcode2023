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

use regex::{Regex};

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
    let reds = 12;
    let greens = 13;
    let blues = 14;

    let re = Regex::new(r"^Game (?<id>\d+)").unwrap();
    let mut good_games = Vec::new();
    for line in INPUT.lines() {
        let Some(caps) = re.captures(line) else {
            continue;
        };
        let id : u32 = caps["id"].parse().unwrap();

        let game = parse(line);
        let mut valid = true;
        for draw in game {
            for balls in draw {
                valid &= match balls {
                    (num, Color::Red) => reds >= num,
                    (num, Color::Green) => greens >= num,
                    (num, Color::Blue) => blues >= num,
                }
            }
            if !valid {break;}
        }
        if valid {
            good_games.push(id);
        }
    }
    println!("Good games: {good_games:#?}");
    let sum : u32 = good_games.iter().sum();
    println!("Sum = {sum}");
}
