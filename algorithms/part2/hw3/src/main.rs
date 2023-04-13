use std::fs;

use hw3::{HeapMM, MedianMaintainer};

fn main() {
    let mut data: HeapMM<i32> = HeapMM::new();
    let mut running_total = 0;
    let median_data = fs::read_to_string("Median.txt")
        .expect("Couldn't read filename");

    for line in median_data.lines() {
        data.push(line.parse().unwrap());
        if let Some(median) = data.peek() {
            running_total += median
        }
    }

    println!("{}", running_total)
}
