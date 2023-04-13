use std::{fs, collections::BinaryHeap};

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    destination: usize,
    dist: usize,
}


impl Edge {
    fn from_str(data_string: &str) -> Edge {
        let edge_data: Vec<usize> = data_string
            .split(",")
            .map(|s|
                s.parse().expect("Expected a string of the form: 'destination,dist'"))
            .collect();
        
        Edge { destination: edge_data[0], dist: edge_data[1] }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| other.destination.cmp(&self.destination))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq)]
pub struct AdjacencyList(Vec<Vec<Edge>>);

impl AdjacencyList {

    pub fn parse_adjacencylist(filename: &str) -> AdjacencyList {
        let file_data = fs::read_to_string(filename)
            .expect("Couldn't read file.");
    
        let mut adj_list: Vec<Vec<Edge>> = vec![vec![]];
        for line in file_data.lines() {
            let mut split_line = line.split_ascii_whitespace();
            let mut edges = vec![];
            split_line.next();
            for edge in split_line {
                edges.push(Edge::from_str(edge))
            }
            adj_list.push(edges)
        }
    
        AdjacencyList(adj_list)
    }

    pub fn num_nodes(&self) -> usize {
        let AdjacencyList(adj_list ) = &self;
        adj_list.len()
    }

    // Implements Dijkstra's algorithm. Returns a vector of distances, where
    // result[i] is the distance from the source node to node i. Distance is MAX_DIST
    // if node is not reachable from source
    pub fn dijkstra(&self, source: usize) -> Vec<usize> {
        const MAX_DIST: usize = 1_000_000;
        let mut distances = vec![MAX_DIST; self.num_nodes()];
        let mut node_queue: BinaryHeap<Edge> = BinaryHeap::new();

        let AdjacencyList(adjacencies) = self;

        for edge in &adjacencies[source] {
            node_queue.push(Edge{ destination: edge.destination, dist: edge.dist })
        }

        while let Some(Edge { destination, dist }) = node_queue.pop() {
            if dist < distances[destination] {
                distances[destination] = dist;
                update_distances(adjacencies, &mut node_queue, &mut distances, destination,
                    dist)   
            }
        }

        distances
    }


}

fn update_distances(adjacencies: &Vec<Vec<Edge>>, node_queue: &mut BinaryHeap<Edge>, 
    distances: &mut Vec<usize>, start: usize, path_dist: usize) {

    for edge in &adjacencies[start] {
        let new_path_dist = edge.dist + path_dist;
        if new_path_dist < distances[edge.destination] {
            //distances[edge.destination] = new_path_dist;
            node_queue.push(Edge { destination: edge.destination, dist: new_path_dist })
        }
    }
}





#[cfg(test)]
mod tests {
    use crate::{AdjacencyList, Edge};

    fn init_list1 () -> AdjacencyList {
        let adjacencies = vec![
            vec![], 
            vec![Edge {destination: 2, dist: 30}, Edge {destination: 3, dist: 12}], 
            vec![Edge {destination: 3, dist: 40}, Edge {destination: 1, dist: 10}], 
            vec![Edge {destination: 1, dist: 2}]
        ];
        AdjacencyList(adjacencies)
    }

    #[test]
    fn test_parser() {
        let graph = init_list1();
        assert_eq!(graph, 
            AdjacencyList::parse_adjacencylist("testfiles/test1.txt"))
    }

    #[test]
    fn test_dijkstra() {
        let graph = init_list1();
        assert_eq!(graph.dijkstra(1), vec![1_000_000, 14, 30, 12]);
        assert_eq!(graph.dijkstra(2), vec![1_000_000, 10, 40, 22]);
        assert_eq!(graph.dijkstra(3), vec![1_000_000, 2, 32, 14]);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Edge::from_str("2,30"), Edge{destination: 2, dist: 30});
        assert_eq!(Edge::from_str("3,12"), Edge{destination: 3, dist: 12});
        assert_eq!(Edge::from_str("3,40"), Edge{destination: 3, dist: 40});
    }

    #[test]
    fn test_edge_comparisons() {
        assert_eq!(Edge{ destination: 2, dist: 30} < Edge{ destination: 3, dist: 12}, true);
        assert_eq!(Edge{ destination: 2, dist: 30} >= Edge{ destination: 3, dist: 30}, true);
        assert_eq!(Edge{ destination: 2, dist: 1} > Edge{ destination: 3, dist: 30}, true)
    }

}