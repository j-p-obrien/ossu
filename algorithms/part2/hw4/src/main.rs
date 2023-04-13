use std::{collections::HashSet, fs};

fn has_distinct_sum(target: i64, numbers: &HashSet<i64>) -> bool {
    for number in numbers {
        let candidate = target - number;
        if numbers.contains(&candidate) && candidate != *number {
            return true;
        }
    }
    return false;
}

fn main() {
    let mut distinct_sums = vec![];
    let numbers: HashSet<i64> = fs::read_to_string("2sum.txt")
        .expect("Couldn't read file.")
        .lines()
        .map(|i| i.parse().expect("Couldn't parse input"))
        .collect();

    for target in -10000..10001 {
        if target % 100 == 0 {
            println!("{}", target)
        }
        if has_distinct_sum(target, &numbers) {
            distinct_sums.push(target)
        }
    }
    println!(
        "There are {} targets with distinct summands.",
        distinct_sums.len()
    )
}
