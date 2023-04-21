use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    str::FromStr,
};

type Vertex = u32;
type Cost = i32;

// Edge for an edge list
#[derive(Debug)]
pub struct Edge {
    pub from: Vertex,
    pub to: Vertex,
    pub cost: Cost,
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

// impl these because the Heap we're using is a max-heap
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

// Since the edges are undirected, we need to implement this manually
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        if self.cost != other.cost {
            return false;
        }
        if (self.from == other.from) & (self.to == other.to) {
            return true;
        }
        if (self.from == other.to) & (self.to == other.from) {
            return true;
        }
        false
    }
}

impl Eq for Edge {}

impl Edge {
    fn from(vertex: &Vertex, al_edge: &AdjacencyListEdge) -> Edge {
        Edge {
            from: *vertex,
            to: al_edge.to,
            cost: al_edge.cost,
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
    // Expects a file with an arbitrary line followed by lines of the form:
    // '[u32] [u32] [i32]'
    pub fn parse_file(filename: &str) -> Self {
        let file_data = fs::read_to_string(filename).expect("Couldn't read file");
        let adjacency_list = AdjacencyList::from_str(&file_data).expect("File had wrong format");
        adjacency_list
    }

    // Adds an edge to the AdjacencyList. Graph is undirected so we must
    // push each edge twice
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

    // Implements Prim's Minimum Spanning Tree algorithm. Returns Some vector
    // of Edges in the spanning tree, or None if graph is empty. If graph is disconnected, returns
    // returns Some vector of Edges that span an arbitrary connected subset of the tree.
    pub fn prims_mst(&self) -> Option<Vec<Edge>> {
        // Get an arbitrary key as starting vertex and put rest into a HashSet. Return None if
        // graph is empty.
        let (start_vertex, mut remaining_vertices) = self.init_vertices()?;

        // Holds the edges of the spanning tree. This is what we will return.
        let mut spanning_edges: Vec<Edge> = vec![];

        // Holds Edges with priority defined by cost. Normally, the only vertices in here would
        // be those that aren't in remaining_vertices, but we can't easily delete from the priority
        // queue, so we will just check to see if they are in remaining_vertices.
        let mut edge_queue: BinaryHeap<Edge> = BinaryHeap::new();
        self.add_valid_edges_to_queue(&start_vertex, &mut edge_queue, &mut remaining_vertices);

        while !remaining_vertices.is_empty() {
            if let Some(edge) = edge_queue.pop() {
                self.process_edge(
                    edge,
                    &mut edge_queue,
                    &mut spanning_edges,
                    &mut remaining_vertices,
                )
            } else {
                // This is only reached if the graph is not connected.
                return Some(spanning_edges);
            }
        }
        Some(spanning_edges)
    }

    // Given a vertex, adds all valid (i.e. `to` vertex is not already in spanning tree) outgoing
    // edges to the edge queue.
    fn add_valid_edges_to_queue(
        &self,
        vertex: &Vertex,
        edge_queue: &mut BinaryHeap<Edge>,
        remaining_vertices: &mut HashSet<Vertex>,
    ) {
        if let Some(al_edges) = self.0.get(vertex) {
            al_edges
                .iter()
                .filter(|al_edge| remaining_vertices.contains(&al_edge.to))
                .for_each(|al_edge| edge_queue.push(Edge::from(vertex, al_edge)))
        }
    }

    // Adds the edge to the spanning tree if it is valid (i.e. `to` vertex is not already
    // contained in the spanning tree). If edge is added, updates the edge queue with its valid
    // outgoing edges, and removes itself from the remaining_vertices set.
    fn process_edge(
        &self,
        edge: Edge,
        edge_queue: &mut BinaryHeap<Edge>,
        spanning_edges: &mut Vec<Edge>,
        remaining_vertices: &mut HashSet<Vertex>,
    ) {
        // If the vertex this Edge points to is contained in remaining_vertices, this returns true and
        // we remove it.
        let candidate_vertex = edge.to;
        let edge_is_valid = remaining_vertices.remove(&candidate_vertex);
        // push Edge to spanning_edges and add valid outgoing edges to the edge queue.
        if edge_is_valid {
            spanning_edges.push(edge);
            self.add_valid_edges_to_queue(&candidate_vertex, edge_queue, remaining_vertices)
        }
    }

    // Picks an arbitrary starting vertex and returns that and a HashSet of the remaining vertices.
    fn init_vertices(&self) -> Option<(Vertex, HashSet<Vertex>)> {
        let mut key_iter = self.0.keys().copied();
        if let Some(vertex) = key_iter.next() {
            let remaining_vertices = key_iter.collect();
            return Some((vertex, remaining_vertices));
        }
        None
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
    fn test_prims() {
        let graph = setup_adj_list();
        let correct_tree = vec![
            Edge {
                from: 1,
                to: 2,
                cost: -2,
            },
            Edge {
                from: 2,
                to: 3,
                cost: 1,
            },
        ];
        let mut spanning_tree = graph.prims_mst().expect("Didn't expect None");
        spanning_tree.sort_by_key(|edge| edge.cost);
        assert_eq!(correct_tree, spanning_tree)
    }
}
