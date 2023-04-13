use hw2::*;
fn main() {
    let adj_list = AdjacencyList::parse_adjacencylist("dijkstraData.txt");
    let indices_of_interest = vec![7,37,59,82,99,115,133,165,188,197];
    let shortest_distances = adj_list.dijkstra(1);
    let mut result = vec![];
    for i in indices_of_interest {
        result.push(shortest_distances[i])
    }
    println!("{:?}", result)
}
