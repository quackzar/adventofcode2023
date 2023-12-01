use std::io::Read;

const INPUT : &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";



const DIGITS : [&str; 9] = [
   "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn str_to_digits(s: &str) -> String {
    let mut buf = s.to_string();
    for (i,d) in DIGITS.iter().enumerate() {
        let i = i + 1;
        buf = buf.replace(d, &format!("{d}{i}{d}"));
    }
    buf
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut results = Vec::new();

    for line in input.lines() {
        let new_line = str_to_digits(line);
        let mut numbers = new_line.chars()
            .filter(|c| c.is_ascii_digit());
        let fst = numbers.next();
        let snd = numbers.last().or(fst);
        let Some(fst) = fst else {
            continue;
        };
        let snd = snd.unwrap();
        let numbers = format!("{fst}{snd}");

        let num : Result<u32,_> = numbers.parse();
        if let Ok(num) = num {
            results.push(num);
        }
    }

    println!("{results:#?}");

    let sum = results.iter().sum::<u32>();
    println!("{sum}");
}
