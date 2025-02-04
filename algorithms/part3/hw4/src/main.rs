use std::fs;

mod knapsack;

use knapsack::Knapsack;

fn main() {
    let knapsack_data = fs::read_to_string("knapsack1.txt").unwrap();
    let knapsack = Knapsack::from_str(&knapsack_data);
    println!(
        "Max value of the first knapsack is: {}",
        knapsack.max_value()
    );

    let big_knapsack_data = fs::read_to_string(&"knapsack_big.txt").unwrap();
    let big_knapsack = Knapsack::from_str(&big_knapsack_data);
    println!(
        "Max value of the big knapsack is: {}",
        big_knapsack.max_value()
    );
}
