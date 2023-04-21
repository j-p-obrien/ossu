use std::{collections::HashMap, fs, str::FromStr};

type Vertex = u32;
type Cost = i32;

// Edge for an edge list
#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    from: Vertex,
    to: Vertex,
    cost: Cost,
}

// Edge for an adjacency list. Users should not use this struct; it is only an implementation
// detail for the AdjacencyList struct.
#[derive(Debug, PartialEq, Eq)]
struct AdjacencyListEdge {
    to: Vertex,
    cost: Cost,
}

// An adjacency list.
#[derive(Debug, PartialEq, Eq)]
pub struct AdjacencyList(HashMap<Vertex, Vec<AdjacencyListEdge>>);

#[derive(Debug)]
pub struct ParseEdgeError;

impl FromStr for Edge {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [Ok(from), Ok(to), Ok(cost)] = s
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Vec<Result<Cost, _>>>()[..]
        {
            Ok(Edge {
                from: from as Vertex,
                to: to as Vertex,
                cost,
            })
        } else {
            Err(ParseEdgeError)
        }
    }
}

impl Edge {
    // DEPRECATED
    fn process_line(line: &str) -> Edge {
        if let [Ok(from), Ok(to), Ok(cost)] = line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Vec<Result<Cost, _>>>()[..]
        {
            Edge {
                from: from as Vertex,
                to: to as Vertex,
                cost,
            }
        } else {
            panic!("Expected a string with types 'u32 u32 i32'")
        }
    }
}

impl FromStr for AdjacencyList {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adjacency_list = AdjacencyList::new();

        // Skip first line of file since it contains irrelevant information.
        let mut lines = s.lines();
        lines.next();

        // Add edges line by line to adjacency list.
        for line in lines {
            let edge = Edge::from_str(line)?;
            adjacency_list.push_edge(edge);
        }

        Ok(adjacency_list)
    }
}

impl AdjacencyList {
    // Returns an empty AdjacencyList.
    pub fn new() -> AdjacencyList {
        AdjacencyList(HashMap::new())
    }

    // Reads file filename and returns an AdjacencyList
    pub fn parse_file(filename: &str) -> Self {
        let file_data = fs::read_to_string(filename).expect("Couldn't read file");
        let adjacency_list = AdjacencyList::from_str(&file_data).expect("File had wrong format");
        adjacency_list
    }

    // Adds an edge to the AdjacencyList.
    pub fn push_edge(&mut self, edge: Edge) {
        let adjacency_list = &mut self.0;
        let Edge { from, to, cost } = edge;

        // push edge going from `from` to `to`
        adjacency_list
            .entry(from)
            .or_insert(vec![])
            .push(AdjacencyListEdge { to: to, cost: cost });

        // push edge going from `to` to `from`
        adjacency_list
            .entry(to)
            .or_insert(vec![])
            .push(AdjacencyListEdge {
                to: from,
                cost: cost,
            })
    }

    // Implements Prim's Minimum Spanning Tree algorithm. Returns a vector
    // of Edges in the spanning tree.
    pub fn prims_mst(&self) -> Vec<Edge> {
        if let Some(min_cost_edge) = self.get_min_cost_edge() {}

        todo!()
    }

    // Gets the edge with the lowest cost. We will start Prim's algorithm with this
    // edge.
    pub fn get_min_cost_edge(&self) -> Option<Edge> {
        let adjacency_list = &self.0;

        adjacency_list
            .iter()
            .filter_map(|(vertex, edges)| Self::min_outgoing_edge(vertex, edges))
            .min_by_key(|edge| edge.cost)
    }

    // Returns an arbitrary Edge with the lowest cost from the associated AdjacencyList.
    fn min_outgoing_edge(vertex: &Vertex, edges: &Vec<AdjacencyListEdge>) -> Option<Edge> {
        if let Some(min_cost_edge) = edges.iter().min_by_key(|edge| edge.cost) {
            return Some(Edge {
                from: *vertex,
                to: min_cost_edge.to,
                cost: min_cost_edge.cost,
            });
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_adj_list() -> AdjacencyList {
        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(
            1,
            vec![
                AdjacencyListEdge { to: 2, cost: -2 },
                AdjacencyListEdge { to: 3, cost: 2 },
            ],
        );
        adjacency_list.insert(
            2,
            vec![
                AdjacencyListEdge { to: 1, cost: -2 },
                AdjacencyListEdge { to: 3, cost: 1 },
            ],
        );
        adjacency_list.insert(
            3,
            vec![
                AdjacencyListEdge { to: 2, cost: 1 },
                AdjacencyListEdge { to: 1, cost: 2 },
            ],
        );
        AdjacencyList(adjacency_list)
    }

    #[test]
    fn test_edge_push() {
        let mut adjacency_list = AdjacencyList::new();
        adjacency_list.push_edge(Edge {
            from: 1,
            to: 2,
            cost: -2,
        });
        adjacency_list.push_edge(Edge {
            from: 2,
            to: 3,
            cost: 1,
        });
        adjacency_list.push_edge(Edge {
            from: 1,
            to: 3,
            cost: 2,
        });
        assert_eq!(adjacency_list, setup_adj_list())
    }

    #[test]
    fn test_parser() {
        let adjacency_list = AdjacencyList::parse_file("testfile2.txt");

        assert_eq!(adjacency_list, setup_adj_list())
    }

    #[test]
    fn test_min_edge_cost() {
        let adjacency_list = setup_adj_list();
        assert_eq!(
            adjacency_list.get_min_cost_edge(),
            Some(Edge {
                from: 1,
                to: 2,
                cost: -2
            })
        )
    }
}
