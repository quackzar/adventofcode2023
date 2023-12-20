use std::{io::Read, collections::{VecDeque, BTreeMap, BTreeSet, HashMap, HashSet}, fmt::Display, time::Duration};
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
    let visited = HashMap::new();
    Map { width, data, start_point, visited, mask: None }
}

struct Map {
    width: usize,
    data: Box<[Pipe]>,
    visited: HashMap<(usize, usize), u16>,
    start_point: (usize, usize),
    mask: Option<HashSet<(usize, usize)>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let use_mask = self.mask.is_some();
        for (i, pipe) in self.data.iter().enumerate() {
            if i % self.width == 0 {
                writeln!(f)?;
            }

            if use_mask && self.mask.as_ref().unwrap().contains(&(i % self.width, i / self.width)) {
                write!(f, "X")?;
                continue;
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
            .filter(|p| !self.is_visited(**p))
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

    fn quadrant(&self, (x,y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let x = x as isize;
        let y = y as isize;

        [(x-1,y), (x+1, y), (x, y-1), (x, y+1)].into_iter()
            .filter(|p| self.is_inside_map(*p))
            .map(|(x,y)| (x as usize, y as usize))

    }
}


fn solve1(input: &str) -> u32{
    let mut map = parse(input);
    let start_pos = map.start_point;
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_pos, 0));
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

    println!("Partitioning...");
    let (left, right) = partition(&map);
    
    println!("Filling left...");
    let left = fill(&map, &left);
    map.mask = Some(left.clone());
    println!("{map}");

    println!("Filling right...");
    let right = fill(&map, &right);
    map.mask = Some(right.clone());
    println!("{map}");

    dbg!(left.len());
    dbg!(right.len());
    right.len() as u32
}

fn fill(map: &Map, seeds: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut to_visit = VecDeque::from_iter(seeds.iter().cloned());
    let mut visisted = HashSet::new();
    while let Some(pos) = to_visit.pop_front() {
        if visisted.contains(&pos) {
            continue;
        }
        visisted.insert(pos);
        let neighbours = map.quadrant(pos);
        for p in neighbours.filter(|p| !map.is_visited(*p)) {
            to_visit.push_back(p);
        }
    };
    visisted
}

#[derive(PartialEq, Eq)]
enum Direction {
    North, South, East, West
}

fn partition(map: &Map) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>){
    let mut left = HashSet::new();
    let mut right = HashSet::new();

    let mut direction = Direction::North;
    let mut pos = map.start_point;
    let mut visisted = HashSet::new();
    loop {
        let pipe = map.get(pos);
        visisted.insert(pos);
        if !map.is_visited(pos) {
            panic!("We are at a bad place");
        }
        let (x,y) = pos;
        match pipe {
            Pipe::Start => {
                dbg!("Visited start!");
                let Some(&p) = map
                    .maybe_neighbours(pos)
                    .iter()
                    .filter(|p| map.is_visited(**p))
                    .find(|p| !visisted.contains(*p))
                else {
                    break;
                };
                pos = p;
                if pos.0 < map.start_point.0 {
                    direction = Direction::West
                } else if pos.0 > map.start_point.0 {
                    direction = Direction::East
                } else if pos.1 < map.start_point.1 {
                    direction = Direction::North
                } else if pos.1 > map.start_point.1 {
                    direction = Direction::South
                } else {
                    panic!("oops");
                }
            },
            Pipe::NorthSouth => {
                if direction == Direction::South {
                    pos = (x, y + 1);
                } else { // Direction is North
                    pos = (x, y - 1);
                }
            },
            Pipe::EastWest => {
                if direction == Direction::West {
                    pos = (x - 1, y);
                } else { // Direction is East
                    pos = (x + 1, y);
                }
            },
            Pipe::NorthEast => {
                if direction == Direction::West {
                    direction = Direction::North;
                    pos = (x, y - 1);
                } else { // Direction is South
                    direction = Direction::East;
                    pos = (x + 1, y);
                }
            },
            Pipe::NorthWest => {
                if direction == Direction::East {
                    direction = Direction::North;
                    pos = (x, y - 1);
                } else { // Direction is South
                    direction = Direction::West;
                    pos = (x - 1, y);
                }
            },
            Pipe::SouthWest => {
                if direction == Direction::East {
                    direction = Direction::South;
                    pos = (x, y + 1);
                } else { // Direction is North
                    direction = Direction::West;
                    pos = (x - 1, y);
                }
            },
            Pipe::SouthEast => {
                if direction == Direction::West {
                    direction = Direction::South;
                    pos = (x, y + 1);
                } else { // Direction is North
                    direction = Direction::East;
                    pos = (x + 1, y);
                }
            },
            _ => {},
        };

        let x = x as isize;
        let y = y as isize;
        let [l,r] = match direction {
            Direction::North => [(x-1,y), (x+1,y)],
            Direction::South => [(x+1,y), (x-1,y)],
            Direction::East => [(x,y-1), (x,y+1)],
            Direction::West => [(x,y+1), (x,y-1)],
        };
        let l0 = (l.0 as usize, l.1 as usize);
        let r0 = (r.0 as usize, r.1 as usize);
        if map.is_inside_map(l) && !map.is_visited(l0) {
            left.insert(l0);
        }
        if map.is_inside_map(r) && !map.is_visited(r0) {
            right.insert(r0);
        }
    }
    (left, right)
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
