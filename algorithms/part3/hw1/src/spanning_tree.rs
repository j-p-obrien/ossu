use std::{str::FromStr, collections::HashMap, num::ParseIntError};

type Vertex = u32;
type Cost = i32;

// Edge for an edge list
#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    from: Vertex,
    to: Vertex,
    cost: Cost
}

// Edge for an adjacency list
#[derive(Debug, PartialEq, Eq)]
pub struct AdjacencyListEdge {
    to: Vertex,
    cost: Cost
}

// An adjacency list
#[derive(Debug, PartialEq, Eq)]
pub struct AdjacencyList(HashMap<Vertex, Vec<AdjacencyListEdge>>);

impl Edge {
    fn process_line(line: &str) -> Edge {
        if let [Ok(from), Ok(to), Ok(cost)] = line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Vec<Result<Cost, _>>>()[..] {
                return Edge { from: from as Vertex, to: to as Vertex, cost }
        } else {
            panic!("Expected a string with types 'u32 u32 i32'")
        }
    }
}

impl FromStr for AdjacencyList {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adjacency_list = AdjacencyList::new();

        // Skip first line of file since it contains irrelevant information.
        let mut lines = s.lines();
        lines.next();

        // Add edges line by line to adjacency list. 
        lines.for_each(|line| adjacency_list.push_edge(Edge::process_line(line)));

        Ok(adjacency_list)
    }
}

impl AdjacencyList {
    // Returns an empty AdjacencyList.
    pub fn new() -> AdjacencyList {
        AdjacencyList(HashMap::new())
    }

    // Adds an edge to the AdjacencyList.
    pub fn push_edge(&mut self, edge: Edge) {
        let AdjacencyList(adjacency_list) = self;
        let Edge { from, to, cost } = edge;

        // push edge going from `from` to `to`
        adjacency_list.entry(from)
            .or_insert(vec![])
            .push(AdjacencyListEdge { to: to, cost: cost });

        // push edge going from `to` to `from`
        adjacency_list.entry(to)
            .or_insert(vec![])
            .push(AdjacencyListEdge { to: from, cost: cost })
    }

    

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_adj_list() -> AdjacencyList{
        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(1, vec![
            AdjacencyListEdge { to: 2, cost: -2 },
            AdjacencyListEdge { to: 3, cost: 2 }]);
        adjacency_list.insert(2, vec![
            AdjacencyListEdge { to: 1, cost: -2 },
            AdjacencyListEdge { to: 3, cost: 1 }]);
        adjacency_list.insert(3, vec![
            AdjacencyListEdge { to: 2, cost: 1 },
            AdjacencyListEdge { to: 1, cost: 2 }]);
        AdjacencyList(adjacency_list)
    }

    #[test]
    fn test_edge_push() {
        let mut adjacency_list = AdjacencyList::new();
        adjacency_list.push_edge(Edge { from: 1, to: 2, cost: -2 });
        adjacency_list.push_edge(Edge { from: 2, to: 3, cost: 1 });
        adjacency_list.push_edge(Edge { from: 1, to: 3, cost: 2 });
        assert_eq!(adjacency_list, setup_adj_list())
    }

    #[test]
    fn test_parser() {
        if let Ok(adjacency_list) = 
            AdjacencyList::from_str(&fs::read_to_string("testfile2.txt").unwrap()) {
            assert_eq!(adjacency_list, setup_adj_list())
        } else {panic!("Couldn't parse testfile")}
    }

}
