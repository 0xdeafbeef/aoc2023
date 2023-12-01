// --- Day 1: Trebuchet?! ---
//
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
//
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
//
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
//
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration values?
// use anyhow::{Context, Result};

fn string_to_digits(s: &str) -> Option<u64> {
    let max_window_size: usize = ALL_DIGITS.iter().map(|x| x.len()).max().unwrap();
    let max_window_size = std::cmp::min(max_window_size, s.len());

    let find_char = |x: &[u8], first_char: char| {
        if x.is_empty() {
            return None;
        }

        if first_char.is_ascii_digit() {
            return Some(first_char);
        }
        ALL_DIGITS.iter().enumerate().find_map(|(idx, num)| {
            x.starts_with(num.as_bytes())
                .then_some(char::from(idx as u8 + b'1'))
        })
    };

    let first_char = WindowsSlice::new(s.as_bytes(), max_window_size)
        .find_map(|x| find_char(x, char::from(*x.first().unwrap())))?;

    let last_char = WindowsSlice::new(s.as_bytes(), max_window_size)
        .rev()
        .find_map(|x| find_char(x, char::from(*x.first().unwrap_or(&b' '))))?;

    fn atoi(c: char) -> u64 {
        c as u64 - b'0' as u64
    }

    // Some(format!("{first_char}{last_char}").parse().unwrap())
    Some(atoi(first_char) * 10 + atoi(last_char))
}

const ALL_DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let data = include_str!("../../data/input_day_1");

    let sum = data
        .lines()
        .map(|x| string_to_digits(x).unwrap())
        .sum::<u64>();
    println!("sum  {sum}")
}

// iterator which returns windows of a slice no longer than window_size, starting with one element
struct WindowsSlice<'a, T> {
    slice: &'a [T],
    window_size: usize,
    current_idx: usize,
}

impl<'a, T> WindowsSlice<'a, T> {
    fn new(slice: &'a [T], window_size: usize) -> Self {
        Self {
            slice,
            window_size,
            current_idx: 0,
        }
    }
}

impl<'a, T> Iterator for WindowsSlice<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx > self.slice.len() {
            return None;
        }

        let end = std::cmp::min(self.current_idx + self.window_size, self.slice.len());
        let res = &self.slice[self.current_idx..end];
        self.current_idx += 1;
        Some(res)
    }
}

impl<'a, T> DoubleEndedIterator for WindowsSlice<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_idx > self.slice.len() {
            return None;
        }

        let start = self.slice.len() - self.current_idx;
        let end = (self.slice.len() - self.current_idx + self.window_size).min(self.slice.len());

        let res = &self.slice[start..end];
        self.current_idx += 1;

        Some(res)
    }
}

#[cfg(test)]
mod test {
    use crate::string_to_digits;

    #[test]
    fn test() {
        let test_data = &[
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        assert_eq!(string_to_digits(test_data[0]).unwrap(), 29);
        assert_eq!(string_to_digits(test_data[1]).unwrap(), 83);
        assert_eq!(string_to_digits(test_data[2]).unwrap(), 13);
        assert_eq!(string_to_digits(test_data[3]).unwrap(), 24);
        assert_eq!(string_to_digits(test_data[4]).unwrap(), 42);
        assert_eq!(string_to_digits(test_data[5]).unwrap(), 14);
        assert_eq!(string_to_digits(test_data[6]).unwrap(), 76);
        assert_eq!(string_to_digits("1abc2").unwrap(), 12);
        assert_eq!(string_to_digits("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(string_to_digits("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(string_to_digits("treb7uchet").unwrap(), 77);
        assert_eq!(string_to_digits("onegaa").unwrap(), 11);
        assert_eq!(string_to_digits("45xj").unwrap(), 45);
    }
}
