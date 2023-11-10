use std::fs;

use crate::knapsack::Knapsack;

mod knapsack;

fn main() {
    let knapsack_data = fs::read_to_string("knapsack1.txt").unwrap();
    let knapsack = Knapsack::from(&knapsack_data);
    println!(
        "Max value of the first knapsack is: {}",
        knapsack.max_value()
    );

    let big_knapsack_data = fs::read_to_string(&"knapsack_big.txt").unwrap();
    let big_knapsack = Knapsack::from(&big_knapsack_data);
    println!(
        "Max value of the big knapsack is: {}",
        big_knapsack.max_value()
    );
}
