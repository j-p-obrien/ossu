mod apsp;
use apsp::Graph;
use std::fs;

fn main() {
    let g1 = Graph::from_str(&fs::read_to_string("g1.txt").unwrap());
    let g2 = Graph::from_str(&fs::read_to_string("g2.txt").unwrap());
    let g3 = Graph::from_str(&fs::read_to_string("g3.txt").unwrap());

    let g1_dist = g1.floyd_warshall();
    let g2_dist = g2.floyd_warshall();
    let g3_dist = g3.floyd_warshall();

    let min_dist = g1_dist.min(g2_dist.min(g3_dist));

    println!("Minimum distance path has length: {:?}", min_dist)
}
