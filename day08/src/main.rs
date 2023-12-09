fn main() {
    let data = include_str!("../../data/input_day_9");

    let data = data.lines().map(|x| {
        x.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    let sum: i64 = data.clone().map(|d| process_seq(&d)).sum();
    println!("res 1: {}", sum); //2175229206
    let sum: i64 = data.map(|d| process_seq_back(&d)).sum();
    println!("res 2: {}", sum); //942
}

fn process_seq(data: &[i64]) -> i64 {
    if data.iter().all(|x| *x == 0) {
        return 0;
    }
    let differences = data.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
    process_seq(&differences) + *data.last().unwrap()
}

fn process_seq_back(data: &[i64]) -> i64 {
    if data.iter().all(|x| *x == 0) {
        return 0;
    }
    let differences = data.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
    *data.first().unwrap() - process_seq_back(&differences)
}
