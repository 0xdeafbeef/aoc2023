use std::collections::HashSet;

fn main() {
    #[rustfmt::skip]
    let data :Vec<_>= include_str!("../../data/input_day_4")
        .lines()
        .map(|line| line.split_once(':').unwrap().1.trim().split_once('|').unwrap())
        .map(|(left, right)| (
            left.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<_>>(),
            right.split_whitespace().map(|n| n.parse().unwrap()).collect::<HashSet<_>>(),
        ))
        .map(|(left,right)| left.intersection(&right).count())
        .collect();

    let sum1 = data
        .iter()
        .map(|&n| if n > 1 { 2_usize.pow(n as u32 - 1) } else { n })
        .sum::<usize>();

    let mut stack: Vec<_> = data.iter().map(|_| 1).collect();

    #[rustfmt::skip]
    let sum2 = data.iter().enumerate().fold(0, |acc, (idx, &score)| {
        let num_copies = stack[idx];
        stack.iter_mut().skip(idx + 1).take(score).for_each(|x| *x += num_copies);
        acc + num_copies
    });

    println!("res 1: {}", sum1); //21568
    println!("res 2: {}", sum2); //11827296
}
