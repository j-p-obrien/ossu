use hw2::{hamming::CodeList, union_find::EdgeList};
fn main() {
    let graph = EdgeList::from_file("clustering1.txt");
    let spacing = graph.cluster(4).expect("Expected some spacing");
    println!("The max spacing of the 4-cluster is {}", spacing);

    let graph = CodeList::from_file("clustering_big.txt");
    let num_clusters = graph.cluster(3);
    println!(
        "The number of clusters with spacing of 3 is: {}",
        num_clusters
    )
}
