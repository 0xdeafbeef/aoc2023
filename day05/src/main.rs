fn main() {
    let start = std::time::Instant::now();
    #[rustfmt::skip]
    let process = |line:&str| line.split_whitespace().filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<_>>();
    let data = include_str!("../../data/input_day_6")
        .split_once('\n')
        .into_iter()
        .flat_map(|(left, right)| process(left).into_iter().zip(process(right)));
    #[rustfmt::skip]
    let bruteforce = |(total_time, distance)|
        (1..total_time).filter(|&attempt_time| attempt_time * (total_time - attempt_time) > distance).count() as u64;
    let res_1: u64 = data.clone().map(|(a, b)| bruteforce((a, b))).product();
    println!("res 1: {}", res_1);
    let res_2 = data.fold((0, 0), |(pre_a, pre_b), (a, b)| {
        let int_join = |base: u64, part: u64| base * 10u64.pow(part.ilog10() + 1) + part;
        (int_join(pre_a, a), int_join(pre_b, b))
    });
    let res_2 = bruteforce(res_2);
    println!("res 2: {}", res_2);
    println!("time: {}ms", start.elapsed().as_millis());
}
fn solve(data: &str) {
    let process = |line: &str| {
        line.split_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<_>>()
    };
    let data = data
        .split_once('\n')
        .into_iter()
        .flat_map(|(left, right)| process(left).into_iter().zip(process(right)));
    #[rustfmt::skip]
        let bruteforce = |(total_time, distance)|
        (1..total_time).filter(|&attempt_time| attempt_time * (total_time - attempt_time) > distance).count() as u64;
    let res_1: u64 = data.clone().map(|(a, b)| bruteforce((a, b))).product();
    println!("res 1: {}", res_1);
    let res_2 = data.fold((0, 0), |(pre_a, pre_b), (a, b)| {
        let int_join = |base: u64, part: u64| base * 10u64.pow(part.ilog10() + 1) + part;
        (int_join(pre_a, a), int_join(pre_b, b))
    });
    let res_2 = bruteforce(res_2);
    println!("res 2: {}", res_2);
}

//160816
// 46561107
