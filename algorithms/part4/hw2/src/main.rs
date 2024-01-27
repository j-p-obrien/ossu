use std::time::SystemTime;

use crate::hw2::Cities;

mod hw2;

fn main() {
    let cities = Cities::from_str(&std::fs::read_to_string("tsp.txt").unwrap());

    let start = SystemTime::now();
    let shortest_path = cities.tsp().floor();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
    println!("The shortest path through all cities is: {shortest_path}");
}
