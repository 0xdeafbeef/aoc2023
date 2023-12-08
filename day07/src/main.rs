use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../data/input_day_8");
    let data: HashMap<String, _> = input.split('\n').skip(2).map(process_line).collect();
    let director = EndlessCycleStr::new(input.lines().next().unwrap());
    let res = part_1(&data, &mut director.clone(), "AAA");
    println!("res 1: {}", res);
    let labels_ending_on_a = data
        .iter()
        .filter(|(label, _)| label.ends_with('A'))
        .map(|(label, _)| label.clone())
        .collect::<Vec<_>>();

    let res_2 = part_2(&data, director, &labels_ending_on_a);
    println!("res 2: {}", res_2);
}

fn process_line(line: &str) -> (String, (String, String)) {
    let (left, right) = line.split_once('=').unwrap();
    let left = left.trim().to_owned();
    let process_enclosed = |x: &str| {
        x.chars()
            .filter(|x| x.is_alphanumeric())
            .collect::<String>()
    };
    let process_pair = |(l, r)| (process_enclosed(l), process_enclosed(r));

    let right = right.split_once(',').map(process_pair).unwrap();
    (left, right)
}

fn part_1(
    map: &HashMap<String, (String, String)>,
    mut director: impl Iterator<Item = Direction>,
    label: &str,
) -> u32 {
    if label == "ZZZ" {
        return 0;
    }
    let op = director.next().unwrap();
    let (left, right) = map.get(label).unwrap();
    1 + match op {
        Direction::Left => part_1(map, director, left),
        Direction::Right => part_1(map, director, right),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }
    a * b / gcd(a, b)
}

fn find_cycle_path(
    map: &HashMap<String, (String, String)>,
    director: impl Iterator<Item = Direction>,
    label: &str,
) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    let mut current = label;

    for (count, op) in director.enumerate() {
        if current.ends_with('Z') && !visited.insert((current.to_string(), op)) {
            break;
        }
        if current.ends_with('Z') {
            path.push(count);
        }
        current = match op {
            Direction::Left => &map[current].0,
            Direction::Right => &map[current].1,
        };
    }
    path
}

fn part_2(
    map: &HashMap<String, (String, String)>,
    director: EndlessCycleStr,
    labels: &[String],
) -> u64 {
    let mut paths: Vec<_> = labels
        .iter()
        .map(|label| find_cycle_path(map, director.clone(), label))
        .collect::<Vec<_>>();

    let max_len = paths.iter().map(|x| x.len()).min().unwrap();
    let mut min_lcm = u64::MAX;
    for idx in 0..max_len {
        let current = paths.iter_mut().map(|x| x[idx]);

        let lcm = current.fold(1, |acc, x| lcm(acc, x as u64));
        min_lcm = std::cmp::min(min_lcm, lcm);
    }
    min_lcm
}

#[derive(Debug, Clone)]
struct EndlessCycleStr {
    chars: Vec<char>,
    index: usize,
}

impl EndlessCycleStr {
    fn new(s: &str) -> Self {
        EndlessCycleStr {
            chars: s.chars().collect::<Vec<char>>(),
            index: 0,
        }
    }
}

impl Iterator for EndlessCycleStr {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chars.is_empty() {
            return None;
        }
        let result = Some(Direction::from_char(self.chars[self.index]));
        self.index = (self.index + 1) % self.chars.len();
        result
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}
