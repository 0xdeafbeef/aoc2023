use anyhow::Context;
use std::str::FromStr;

fn main() {
    let data = include_str!("../../data/input_day_2");

    #[derive(Default, Debug)]
    struct Stats {
        possible_games_sum: u32,
        power_sum: u32,
    }

    impl Stats {
        fn update(mut self, game: &Game, idx: u32) -> Self {
            // 12 red_lower cubes, 13 green cubes, and 14 blue cubes
            let red_max = 12;
            let green_max = 13;
            let blue_max = 14;
            let test_result = game
                .red_lower(red_max)
                .and_then(|x| x.green_lower(green_max))
                .and_then(|x| x.blue_lower(blue_max));
            match test_result {
                Some(_) => self.possible_games_sum += idx + 1, // games start from 1
                None => {
                    println!("Game {} failed", idx);
                }
            }

            self.possible_games_sum += 1;
            self.power_sum += game.min_power();
            self
        }
    }

    let sum = data
        .lines()
        .enumerate()
        .fold(Stats::default(), |acc, (idx, line)| {
            let game = Game::from_str(line).unwrap();
            acc.update(&game, idx as u32)
        });
    println!("sum  {}", sum.possible_games_sum);
    println!("power_sum  {}", sum.power_sum);
}

#[derive(Default, Debug)]
struct Game {
    red: Color,
    green: Color,
    blue: Color,
}

#[derive(Default, Debug)]
struct Color {
    max: u32,
}

impl Color {
    fn update(&mut self, value: u32) {
        self.max = std::cmp::max(self.max, value);
    }
}

impl Game {
    fn red_lower(&self, value: u32) -> Option<&Self> {
        (self.red.max <= value).then_some(self)
    }

    fn green_lower(&self, value: u32) -> Option<&Self> {
        (self.green.max <= value).then_some(self)
    }

    fn blue_lower(&self, value: u32) -> Option<&Self> {
        (self.blue.max <= value).then_some(self)
    }

    fn min_power(&self) -> u32 {
        self.red.max * self.green.max * self.blue.max
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut red, mut green, mut blue) = (Color::default(), Color::default(), Color::default());

        let idx = s.find(':').context("No ':' found")?;
        let s = &s[idx + 1..];

        for set in s.split(';') {
            for color in set.split(',') {
                let color = color.trim();
                let value_idx = color.chars().take_while(|c| c.is_ascii_digit()).count();
                let value = color[..value_idx].parse::<u32>()?;
                let color = color[value_idx..].trim();

                match color {
                    "red" => red.update(value),
                    "green" => green.update(value),
                    "blue" => blue.update(value),
                    _ => return Err(anyhow::anyhow!("Unknown color {}", color)),
                }
            }
        }

        Ok(Self { red, green, blue })
    }
}

#[cfg(test)]
mod test {
    use crate::Game;
    use std::str::FromStr;

    #[test]
    fn test() {
        let game = "Game 1: 3 blue, 2 green, 6 red; 17 green, 4 red, 8 blue; 2 red, 1 green, 10 blue; 1 blue, 5 green";
        let game = Game::from_str(game).unwrap();
        assert_eq!(game.red.max, 6);
        assert_eq!(game.green.max, 17);
        assert_eq!(game.blue.max, 10);

        dbg!(game);
        let games = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let games = games
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        macro_rules! test_game {
            ($game:expr, $res:expr) => {
                assert_eq!(
                    Game::from_str($game)
                        .unwrap()
                        .red_lower(max_red)
                        .and_then(|x| x.green_lower(max_green))
                        .and_then(|x| x.blue_lower(max_blue))
                        .is_some(),
                    $res
                );
            };
            () => {};
        }

        test_game!(games[0], true);
        test_game!(games[1], true);
        test_game!(games[2], false);
        test_game!(games[3], false);
        test_game!(games[4], true);
    }
}
