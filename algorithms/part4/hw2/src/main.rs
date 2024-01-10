use crate::hw2::Cities;

mod hw2;

fn main() {
    let cities = Cities::from_str(&std::fs::read_to_string("tsp.txt").unwrap());
    let shortest_path = cities.tsp().floor();
    println!("The shortest path through all cities is: {shortest_path}")
}
