use std::{io::Read, collections::{VecDeque, BTreeMap, BTreeSet}, fmt::Display, time::Duration};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = solve1(&input);
    println!("Part 1 {solution}");
    let solution = solve2(&input);
    println!("Part 2 {solution}");
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Start,
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::Start => '▣',
            Pipe::Ground => ' ',
            Pipe::NorthSouth => '║',
            Pipe::EastWest => '═',
            Pipe::NorthWest => '╝',
            Pipe::NorthEast => '╚',
            Pipe::SouthWest => '╗',
            Pipe::SouthEast => '╔',
            // Pipe::Visited(_) => '▓',
        };
        write!(f, "{c}")
    }
}

fn display_visited(pipe: &Pipe) -> char {

    match pipe {
        Pipe::Start => '▣',
        Pipe::Ground => ' ',
        Pipe::NorthSouth => '┃',
        Pipe::EastWest => '━',
        Pipe::NorthWest => '┛',
        Pipe::NorthEast => '┗',
        Pipe::SouthWest => '┓',
        Pipe::SouthEast => '┏',
        // Pipe::Visited(_) => '▓',
    }
}


fn translate(ugly: &str) -> String {
    ugly.replace('|', "║")
        .replace('-', "═")
        .replace('L', "╚")
        .replace('J', "╝")
        .replace('7', "╗")
        .replace('F', "╔")
        .replace('.', " ")
        .replace('S', "▣")
}

fn parse(input: &str) -> Map {
    let width = match input.find("\r\n") {
        Some(width) => width,
        None => input.find('\n').unwrap(),
    };

    let data : Box<_> = input.lines().flat_map(|s| s.chars().map(|c| match c {
        '|' => Pipe::NorthSouth,
        '-' => Pipe::EastWest,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::Start,
        _ => panic!("Crash and burn"),
    })).collect();

    let (i, _) = data.iter().enumerate().find(|(_, &p)| p == Pipe::Start).unwrap();
    let start_point = (i % width, i / width);
    let visited = BTreeMap::new();
    Map { width, data, start_point, visited }
}

struct Map {
    width: usize,
    data: Box<[Pipe]>,
    visited: BTreeMap<(usize, usize), u16>,
    start_point: (usize, usize),
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, pipe) in self.data.iter().enumerate() {
            if i % self.width == 0 {
                writeln!(f)?;
            }
            if self.is_visited((i % self.width, i / self.width)) {


                let c = display_visited(pipe);
                write!(f, "{c}")?;
            } else {
                write!(f, "{pipe}")?;
            }
        }
        Ok(())
    }
}

impl Map {
    fn get(&self, (x, y): (usize, usize)) -> Pipe {
        self.data[x + y * self.width]
    }


    fn size(&self) -> (usize, usize) {
        (self.width, self.data.len() / self.width)
    }

    fn neighbours(&self, pos: (usize, usize)) -> Box<[(usize, usize)]>{
        let it = self.maybe_neighbours(pos);
        let me = pos;
        it.iter()
            .filter(|&pos| self.maybe_neighbours(*pos).contains(&me))
            .cloned()
            .collect()
    }

    fn maybe_neighbours(&self, pos: (usize, usize)) -> Box<[(usize, usize)]> {
        let pipe = self.get(pos);
        let (x,y) = pos;
        let x = x as isize;
        let y = y as isize;
        let it : Box<[_]> = match pipe {
            Pipe::Start => Box::new([(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]),
            Pipe::Ground => Box::new([]),
            //Pipe::Visited(_) => Box::new([]),
            Pipe::NorthSouth => Box::new([(x, y - 1), (x, y + 1)]),
            Pipe::EastWest => Box::new([(x - 1, y), (x + 1, y)]),
            Pipe::NorthEast => Box::new([(x, y - 1), (x + 1, y)]),
            Pipe::NorthWest => Box::new([(x, y - 1), (x - 1, y)]),
            Pipe::SouthWest => Box::new([(x, y + 1), (x - 1, y)]),
            Pipe::SouthEast => Box::new([(x, y + 1), (x + 1, y)]),
        };
        it.iter().filter(|pos| self.is_inside_map(**pos))
            .map(|(x,y)| (*x as usize, *y as usize))
            .filter(|p| !self.is_visited(*p))
            .collect()  
    }

    fn visit(&mut self, pos: (usize, usize), value: u16) {
        if self.visited.contains_key(&pos) {
            return
        }
        self.visited.insert(pos, value);
    }
    fn is_visited(&self, pos: (usize, usize)) -> bool {
        self.visited.contains_key(&pos)
    }

    fn is_inside_map(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 &&
            pos.1 >= 0 &&
            pos.0 < self.width as isize &&
            pos.1 < (self.data.len() / self.width) as isize 
    }
}


fn solve1(input: &str) -> u32{
    let mut map = parse(input);
    let start_pos = map.start_point;
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_pos, 0));
    let size = map.size();
    while let Some((pos, step)) = to_visit.pop_front() {
        let neighbours = map.neighbours(pos);
        map.visit(pos, step);
        for pos in neighbours.iter() {
            to_visit.push_back((*pos, step + 1));
        }
    };

    *map.visited.values().max().unwrap() as u32
}

fn solve2(input: &str) -> u32 {
    let mut map = parse(input);
    let start_pos = map.start_point;
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_pos, 0));
    let size = map.size();
    while let Some((pos, step)) = to_visit.pop_front() {
        let neighbours = map.neighbours(pos);
        map.visit(pos, step);
        for pos in neighbours.iter() {
            to_visit.push_back((*pos, step + 1));
        }
    };
    println!("{map}");
    let mut seeds : VecDeque<(usize, usize)> = map.visited.keys().flat_map(|p| map.maybe_neighbours(*p).iter().filter(|p| !map.is_visited(**p)).cloned().collect::<Vec<_>>()).collect();
    let mut colored = BTreeSet::new();
    while let Some(seed) = seeds.pop_front() {
        colored.insert(seed);
        println!("{seed:?}");
        map.maybe_neighbours(seed).iter().filter(|p| !map.is_visited(**p) && !colored.contains(*p)).for_each(|p| seeds.push_back(*p));
    }
    
    colored.len() as u32
}


const INPUT: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";


const INPUT2: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

#[test]
fn part1() {
    assert_eq!(solve1(INPUT), 8);
}

#[test]
fn part2() {
    assert_eq!(solve2(INPUT2), 10);
}
