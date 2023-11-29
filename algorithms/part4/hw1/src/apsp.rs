use std::{cmp::Ordering, ops::Add};

type Vertex = usize;
type Dist = isize;

// Edge for adjacency list
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Edge {
    head: Vertex,
    distance: Dist,
}

// Adjacency list
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Graph(Vec<Vec<Edge>>);

// Integers don't have inifinity but it sure would be cooler if they did
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Distance {
    Infinite,
    Finite(Dist),
}

// Contains data on the shortest paths between two vertices
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PathDistances(Vec<Vec<Distance>>);

impl Edge {
    // creates an edge from the given string slice
    pub fn from_str(data: &str) -> Self {
        let (head, distance) = data.split_once(" ").unwrap();
        Self {
            head: head.parse().unwrap(),
            distance: distance.parse().unwrap(),
        }
    }
}

impl Graph {
    // Creates a graph from the given string slice
    pub fn from_str(data: &str) -> Self {
        let mut graph_data = data.lines();
        // who needs error handling lol
        let n_vertices: usize = graph_data
            .next()
            .unwrap()
            .split(" ")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let mut adjacency_list = vec![vec![]; n_vertices + 1];

        for line in graph_data {
            let (tail_data, edge_data) = line.split_once(" ").unwrap();
            let tail: Vertex = tail_data.parse().unwrap();
            let edge = Edge::from_str(edge_data);
            adjacency_list[tail].push(edge);
        }

        Graph(adjacency_list)
    }

    // Returns number of vertices in graph
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len() - 1
    }

    // Returns a vector of tuples where each tuple is (tail, head, distance) for each edge in the
    // graph
    pub fn edges(&self) -> Vec<(Vertex, Vertex, Dist)> {
        self.0
            .iter()
            .enumerate()
            .skip(1)
            .flat_map(|(tail, edges)| {
                edges
                    .iter()
                    .map(move |edge| (tail, edge.head, edge.distance))
            })
            .collect()
    }

    // Computes the all pairs shortest paths for the given Graph. Returns Finite(distance) if
    // There are no negative cycles. Returns Infinite if a negative cycle is detected.
    pub fn floyd_warshall(&self) -> Distance {
        let mut current = PathDistances::init(self);
        let n = self.len();
        for v in 1..=n {
            let last = current;
            current = last.update_distances(v)
        }
        current.min_dist()
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Distance::Infinite, _) => Some(Ordering::Greater),
            (Distance::Finite(_), Distance::Infinite) => Some(Ordering::Less),
            (Distance::Finite(d1), Distance::Finite(d2)) => {
                if d1 == d2 {
                    Some(Ordering::Equal)
                } else if d1 < d2 {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Distance::Finite(left), Distance::Finite(right)) => Distance::Finite(left + right),
            _ => Distance::Infinite,
        }
    }
}

impl PathDistances {
    // Given graph, initializes the distances between vertices according to Floyd-Warahall
    fn init(graph: &Graph) -> Self {
        let n = graph.len();
        let mut weight_data = vec![vec![Distance::Infinite; n + 1]; n + 1];
        for v in 1..=n {
            weight_data[v][v] = Distance::Finite(0);
        }

        for (tail, head, dist) in graph.edges() {
            weight_data[tail][head] = Distance::Finite(dist);
        }
        Self(weight_data)
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len() - 1
    }

    // Given new_vertex, updates shortest path distances to now include paths with interior node
    // new_vertex
    fn update_distances(&self, new_vertex: Vertex) -> PathDistances {
        let mut new = self.clone();
        let n = self.len();
        for i in 1..=n {
            for j in 1..=n {
                new.0[i][j] = self.0[i][j].min(self.0[i][new_vertex] + self.0[new_vertex][j])
            }
        }
        new
    }

    fn detect_neg_cycle(&self) -> bool {
        (1..=self.len())
            .into_iter()
            .any(|v| self.0[v][v] < Distance::Finite(0))
    }

    // Computes the distance of the minimum distance path.
    pub fn min_dist(&self) -> Distance {
        if self.detect_neg_cycle() {
            Distance::Infinite
        } else {
            *self.0.iter().flatten().min().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{Distance, Edge, Graph, PathDistances, Vertex};

    impl Edge {
        fn from(head: Vertex, distance: isize) -> Self {
            Self { head, distance }
        }
    }

    fn graph() -> Graph {
        Graph(vec![
            vec![],
            vec![Edge::from(2, 1)],
            vec![Edge::from(3, -1)],
            vec![Edge::from(1, 2)],
        ])
    }

    fn negcycle_graph() -> Graph {
        Graph(vec![
            vec![],
            vec![Edge::from(2, 1)],
            vec![Edge::from(3, -1), Edge::from(4, 0)],
            vec![Edge::from(1, 2)],
            vec![Edge::from(2, -1)],
        ])
    }

    fn weight_init() -> PathDistances {
        PathDistances(vec![
            vec![
                Distance::Infinite,
                Distance::Infinite,
                Distance::Infinite,
                Distance::Infinite,
            ],
            vec![
                Distance::Infinite,
                Distance::Finite(0),
                Distance::Finite(1),
                Distance::Infinite,
            ],
            vec![
                Distance::Infinite,
                Distance::Infinite,
                Distance::Finite(0),
                Distance::Finite(-1),
            ],
            vec![
                Distance::Infinite,
                Distance::Finite(2),
                Distance::Infinite,
                Distance::Finite(0),
            ],
        ])
    }

    fn weight_init_cycle() -> PathDistances {
        PathDistances(vec![
            vec![
                Distance::Infinite,
                Distance::Infinite,
                Distance::Infinite,
                Distance::Infinite,
                Distance::Infinite,
            ],
            vec![
                Distance::Infinite,
                Distance::Finite(0),
                Distance::Finite(1),
                Distance::Infinite,
                Distance::Infinite,
            ],
            vec![
                Distance::Infinite,
                Distance::Infinite,
                Distance::Finite(0),
                Distance::Finite(-1),
                Distance::Finite(0),
            ],
            vec![
                Distance::Infinite,
                Distance::Finite(2),
                Distance::Infinite,
                Distance::Finite(0),
                Distance::Infinite,
            ],
            vec![
                Distance::Infinite,
                Distance::Infinite,
                Distance::Finite(-1),
                Distance::Infinite,
                Distance::Finite(0),
            ],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            graph(),
            Graph::from_str(&fs::read_to_string(&"testgraph.txt").unwrap())
        )
    }

    #[test]
    fn test_parser_cycle() {
        assert_eq!(
            negcycle_graph(),
            Graph::from_str(&fs::read_to_string(&"testgraph_negcycle.txt").unwrap())
        )
    }

    #[test]
    fn test_weight_init() {
        assert_eq!(PathDistances::init(&graph()), weight_init())
    }

    #[test]
    fn test_weight_init_cycle() {
        assert_eq!(PathDistances::init(&negcycle_graph()), weight_init_cycle())
    }

    #[test]
    fn test_floyd_warshall() {
        assert_eq!(graph().floyd_warshall(), Distance::Finite(-1))
    }

    #[test]
    fn test_floyd_warshall_cycle() {
        assert_eq!(negcycle_graph().floyd_warshall(), Distance::Infinite)
    }
}
