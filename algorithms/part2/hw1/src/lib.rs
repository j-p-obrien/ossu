pub mod edge_list {
    use std::{fs, collections::HashSet};
    use super::adjacency_list::AdjacencyList;

    #[derive(Debug, PartialEq)]
    pub struct EdgeList {
        pub edges: Vec<[i32; 2]>,
    }

    impl EdgeList {
        // Reads in a file containing an edge list and returns an EdgeList.
        pub fn parse_edge_list(filepath: &str) -> EdgeList {
            let f = fs::read_to_string(filepath).expect("Couldn't open file.");

            let edges: Vec<[i32; 2]> = f
                .lines()
                .map(|l| {
                    l.trim_end()
                        .split(" ")
                        .map(|s| s.parse().expect("Expected an i32"))
                        .collect::<Vec<i32>>()
                        .try_into()
                        .expect("Expected two node names per line separated by spaces")
                })
                .collect();
            EdgeList { edges }
        }

        // Reverses the edges in the EdgeList.
        pub fn reverse(&mut self) {
            self
                .edges
                .iter_mut()
                .for_each(|x| {
                    x.reverse()
                })
        }

        // Returns the strongly connected components of the graph.
        // Each element in the first vector is a vector of vertices,
        // where each vector of vertices is a strongly connected
        // component.
        pub fn scc(&mut self) -> Vec<Vec<i32>> {
            let graph = AdjacencyList::from_edge_list(self);
            self.reverse();
            let reversed_graph = AdjacencyList::from_edge_list(self);
            self.reverse();
            let mut visited: HashSet<i32> = HashSet::new();
            let mut finishing_times: Vec<i32> = vec![];

            for vertex in reversed_graph.adjacencies.keys() {
                if !visited.contains(vertex) {
                    if let Some(mut vertices) = 
                        reversed_graph.dfs(vertex, &mut visited) {
                        finishing_times.append(&mut vertices);
                    }
                }
            }

            visited = HashSet::new();
            let mut result: Vec<Vec<i32>> = vec![];

            for vertex in finishing_times.iter().rev() {
                if let Some(visited_vertices) = graph.dfs(vertex, &mut visited) {
                    result.push(visited_vertices);
                }
            }

            result
        }
    }
}

pub mod adjacency_list {
    use super::edge_list::*;
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, PartialEq)]
    pub struct AdjacencyList {
        pub adjacencies: HashMap<i32, Vec<i32>>,
    }

    impl AdjacencyList {
        // Returns the number of nodes (vertices) in the graph.
        pub fn num_nodes(&self) -> usize {
            self.adjacencies.len()
        }

        // Takes an Edge List and returns an Adjacency List.
        pub fn from_edge_list(edge_list: &EdgeList) -> AdjacencyList {
            let mut adjacencies = HashMap::new();
            for edge in &edge_list.edges {
                // If starting node is already in graph, then add ending
                // node; else, add the starting node to the graph along
                // with the ending node.
                adjacencies
                    .entry(edge[0])
                    .and_modify(|vec: &mut Vec<i32>| vec.push(edge[1]))
                    .or_insert(vec![edge[1]]);
                // If ending node is already in map, do nothing;
                // else, insert an empty vector.
                // This allows us to ensure that isolated vertices still
                // have an entry in the adjacency list.
                adjacencies.entry(edge[1]).or_insert(vec![]);
            }
            AdjacencyList { adjacencies }
        }

        // Performs Depth-First Search on self. If the first node has already been visited or
        // doesn't exist in the graph, returns None; else, returns Some vector of
        // all visited nodes ordered by finishing time.
        pub fn dfs(&self, start: &i32, visited: &mut HashSet<i32>) -> Option<Vec<i32>> {
            // Helper function to process todo list. If first node in next has been visited,
            // do nothing and drop it from next. If it has not been visited yet, push itself and
            // its outgoing edges to the top of the todo list and add to visited. If next is empty,
            // we are finished processing vertex and may add it to finishing times.
            fn process_todo<'a>(
                current_vertex: i32,
                next_vertices: &mut &'a [i32],
                todo: &mut Vec<(i32, &'a [i32])>,
                visited: &mut HashSet<i32>,
                finishing_times: &mut Vec<i32>,
                adjacencies: &'a HashMap<i32, Vec<i32>>,
            ) {
                match next_vertices {
                    // If there are no outgoing edges to process, this vertex is finished and so
                    // we add it to finishing_times.
                    &mut &[] => finishing_times.push(current_vertex),
                    // If there is at least one outgoing edge to process, we must check whether it
                    // has been visited or not. First we push the current vertex and its remaining
                    // outgoing edges back onto the todo list (we have to do this because we
                    // can't modify todo both in the calling match expression and also this
                    // function). Then we check if next_vertex has been visited. If it has, we do
                    // nothing; else, we push it and its outgoing edges to the todo list.
                    &mut [next_vertex, rest_vertices @ ..] => {
                        // Push our current vertex and its remaining outgoing edges back to todo.
                        todo.push((current_vertex, rest_vertices));
                        // If next_vertex hasn't already been visited, add it and its outgoing
                        // edges to the todo list.
                        if visited.insert(*next_vertex) {
                            if let Some((&next, next_outgoing)) =
                                adjacencies.get_key_value(next_vertex)
                            {
                                todo.push((next, &next_outgoing[..]))
                            }
                        }
                    }
                }
            }

            // If start is already visited, whole expression evaluates to true and we return None.
            if !visited.insert(*start) {
                return None;
            };

            let adjacencies = &self.adjacencies;
            // If start is not contained in the adjacency list, then return None; else,
            // initialize todo list with starting vertex and its outgoing edges.
            let mut todo = vec![match adjacencies.get_key_value(&start) {
                None => return None,
                Some((&vertex, edges)) => (vertex, &edges[..]),
            }];
            let mut finishing_times: Vec<i32> = vec![];

            loop {
                match todo.pop() {
                    None => break,
                    Some((current_vertex, mut next_vertices)) => process_todo(
                        current_vertex,
                        &mut next_vertices,
                        &mut todo,
                        visited,
                        &mut finishing_times,
                        adjacencies,
                    ),
                }
            }
            return Some(finishing_times);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adjacency_list::*;
    use crate::edge_list::*;
    use std::collections::{HashMap, HashSet};
    use std::vec;

    fn setup_edge_list() -> EdgeList {
        EdgeList {
            edges: vec![[1, 2], [1, 3]],
        }
    }
    fn setup_reversed_edgelist() -> EdgeList {
        EdgeList {
            edges: vec![[2, 1], [3, 1]],
        }
    }
    fn setup_adj_list() -> AdjacencyList {
        let mut adjacencies = HashMap::new();
        adjacencies.insert(1, vec![2, 3]);
        adjacencies.insert(2, vec![]);
        adjacencies.insert(3, vec![]);
        AdjacencyList { adjacencies }
    }

    #[test]
    fn test_parser() {
        let edges: EdgeList = EdgeList::parse_edge_list("test_file.txt");

        assert_eq!(edges, setup_edge_list())
    }

    #[test]
    fn test_reverse() {
        let mut edges = setup_edge_list();
        edges.reverse();
        assert_eq!(edges, setup_reversed_edgelist())
    }

    #[test]
    fn test_from_edge_list() {
        assert_eq!(
            AdjacencyList::from_edge_list(&setup_edge_list()),
            setup_adj_list()
        )
    }

    #[test]
    fn test_dfs_1() {
        let adj_list = setup_adj_list();
        assert_eq!(adj_list.dfs(&1, &mut HashSet::new()), Some(vec![2, 3, 1]));
        assert_eq!(adj_list.dfs(&2, &mut HashSet::new()), Some(vec![2]));

        let mut visited: HashSet<i32> = HashSet::new();
        visited.insert(2);
        assert_eq!(adj_list.dfs(&1, &mut visited), Some(vec![3, 1]));
        assert_eq!(adj_list.dfs(&1, &mut visited), None);

        assert_eq!(adj_list.dfs(&4, &mut HashSet::new()), None)
    }

    #[test]
    fn test_scc() {
        let mut edge_list = setup_edge_list();
        assert_eq!(edge_list.scc(), vec![vec![3], vec![2], vec![1]]);

        let mut edge_list_w_cycle = EdgeList::parse_edge_list("test_file2.txt");
        let mut scc = edge_list_w_cycle.scc();
        scc.iter_mut().for_each(|x| x.sort());
        scc.sort_by_key(|x| x[0]);
        assert_eq!(scc, vec![vec![1, 2], vec![3]])
    }
}
