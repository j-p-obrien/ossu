mod hw3;

fn main() {
    let cities = hw3::Cities::from_str(&std::fs::read_to_string("nn.txt").unwrap());
    let shortest_distance = cities.greedy_tour().floor();
    println!("The greedy tour produced a tour of length: {shortest_distance}.")
}
