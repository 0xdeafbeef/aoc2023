use std::collections::HashMap;

fn main() {
    let data: Vec<Vec<char>> = include_str!("../../data/input_day_3")
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    let (res1, res2) = solve(&data);
    println!("res 1: {}. res 2: {}", res1, res2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self, max_x: usize, max_y: usize) -> impl Iterator<Item = Coord> {
        let x = self.x as isize;
        let y = self.y as isize;
        let positions = [
            (x - 1, y),     // Left
            (x + 1, y),     // Right
            (x, y - 1),     // Up
            (x, y + 1),     // Down
            (x - 1, y - 1), // Top-left
            (x + 1, y - 1), // Top-right
            (x - 1, y + 1), // Bottom-left
            (x + 1, y + 1), // Bottom-right
        ];

        positions.into_iter().filter_map(move |(dx, dy)| {
            if dx >= 0 && dx < max_x as isize && dy >= 0 && dy < max_y as isize {
                Some(Coord {
                    x: dx as usize,
                    y: dy as usize,
                })
            } else {
                None
            }
        })
    }
}

fn solve(data: &[Vec<char>]) -> (usize, usize) {
    let max_x = data[0].len();
    let max_y = data.len();

    let mut sum = 0;
    let mut y = 0;
    let mut gears_map = HashMap::new();

    while y < max_y {
        let mut x = 0;
        while x < max_x {
            let symbol = data[y][x];

            if symbol.is_ascii_digit() {
                let mut number_end = x;
                while number_end < max_x && data[y][number_end].is_ascii_digit() {
                    number_end += 1;
                }

                let number: usize = data[y][x..number_end]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let mut has_symbol_neighbor = false;
                let mut has_star_neighbor = false;

                for nx in x..number_end {
                    for neighbour in Coord::new(nx, y).neighbors(max_x, max_y) {
                        let neighbour_data = data[neighbour.y][neighbour.x];
                        if !(neighbour_data.is_ascii_digit() || neighbour_data == '.') {
                            has_symbol_neighbor = true;
                        }
                        if neighbour_data == '*' && !has_star_neighbor {
                            gears_map.entry(neighbour).or_insert(vec![]).push(number);
                            has_star_neighbor = true;
                        }
                        if has_symbol_neighbor && has_star_neighbor {
                            break;
                        }
                    }
                }

                if has_symbol_neighbor {
                    sum += number;
                }

                // Skip number
                x = number_end;
            } else {
                x += 1;
            }
        }
        y += 1;
    }

    let gears_ratio = gears_map
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().product::<usize>())
        .sum();

    (sum, gears_ratio)
}
