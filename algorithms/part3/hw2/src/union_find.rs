use std::{cell::RefCell, collections::HashSet, fs, str::FromStr};

// Vertices range from 1 to some maximum.
type Vertex = usize;
// Distances are non-negative integers.
type Dist = usize;

// An Edge in a graph
#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    from: Vertex,
    to: Vertex,
    dist: Dist,
}

// An EdgeList. Contains Edges mentioned above.
#[derive(Debug, PartialEq, Eq)]
pub struct EdgeList(RefCell<Vec<Edge>>);

#[derive(Debug)]
pub struct ParseEdgeError;

impl EdgeList {
    // Computes max-spacing k-clustering using Kruskal's MST algorithm. Returns Some(spacing)
    // where spacing is the minimum distance between all points in different clusters. If every
    // point belongs to one cluster, return None.
    pub fn cluster(&self, k: usize) -> Option<Dist> {
        let vertices = self.get_vertices();
        let mut clusters = UnionFind::from(vertices);

        let mut edges = self.0.borrow_mut();
        edges.sort_unstable_by_key(|edge| edge.dist);

        for edge in &*edges {
            if clusters.num_clusters > k {
                clusters.union(edge.from, edge.to)
            } else if clusters.different_clusters(edge) {
                return Some(edge.dist);
            }
        }
        None
    }

    // Returns a vector of the vertices in the graph.
    pub fn get_vertices(&self) -> Vec<Vertex> {
        let v_set = HashSet::new();
        self.0
            .borrow()
            .iter()
            .fold(v_set, |mut acc, edge| {
                acc.insert(edge.from);
                acc.insert(edge.to);
                acc
            })
            .into_iter()
            .collect()
    }

    // Parses a file and creates a new EdgeList.
    // Expects the format:
    // [number of vertices]
    // [from vertex] [to vertex] [distance between vertices]
    // [from vertex] [to vertex] [distance between vertices]
    // ...
    pub fn from_file(path: &str) -> EdgeList {
        let filedata = fs::read_to_string(path).expect("Couldn't read file.");
        let graph = EdgeList::from_str(&filedata).expect("File couldn't be parsed.");
        graph
    }
}

// Creates an EdgeList from the given string slice.
// Expects the format:
// [number of vertices]
// [from vertex] [to vertex] [distance between vertices]
// [from vertex] [to vertex] [distance between vertices]
// ...
impl FromStr for EdgeList {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(edge_data) = s.lines().skip(1).map(|s| Edge::from_str(s)).collect() {
            return Ok(EdgeList(RefCell::new(edge_data)));
        }
        Err(ParseEdgeError)
    }
}
// Creates an Edge from the given string slice.
// Expects the format:
// [from vertex] [to vertex] [distance between vertices]
impl FromStr for Edge {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edge_data: Result<Vec<Vertex>, _> =
            s.split_ascii_whitespace().map(|s| s.parse()).collect();

        if let Ok(&[from, to, dist]) = edge_data.as_deref() {
            return Ok(Edge { from, to, dist });
        }
        Err(ParseEdgeError)
    }
}

// An entry in the UnionFind data structure. Users don't need to use this.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct UnionFindEntry {
    rank: usize,
    parent: Vertex,
}

// A lazy Union-Find data structure with path compression.
#[derive(Debug, PartialEq, Eq)]
pub struct UnionFind {
    entries: RefCell<Vec<UnionFindEntry>>,
    num_clusters: usize,
}

impl UnionFind {
    // Finds parent of given vertex. Rewires parents of all vertices along the path to the root to
    // point to the root (path compression).
    pub fn find(&self, vertex: Vertex) -> Vertex {
        let mut vertices = self.entries.borrow_mut();
        let mut current_vertex = vertex;
        let mut parent_vertex = vertices[vertex].parent;
        let mut to_rewire = vec![];

        // If the parent of the current vertex is not itself, we must go to its parent's parent
        // to find the root.
        while parent_vertex != current_vertex {
            to_rewire.push(current_vertex);
            current_vertex = parent_vertex;
            parent_vertex = vertices[current_vertex].parent;
        }

        // Rewire parent pointers of all vertices that weren't the root
        for vertex in to_rewire {
            vertices[vertex].parent = parent_vertex
        }

        parent_vertex
    }

    // Merges the given vertices into one cluster by rewiring the parent with lower rank to
    // the parent with higher rank. If they have the same rank, rewires the parent of v1
    // to point to the parent of v2 and increments the rank of the parent of v2 by 1.
    // Does nothing if both vertices are already in the same cluster.
    pub fn union(&mut self, v1: Vertex, v2: Vertex) {
        let parent1 = self.find(v1);
        let parent2 = self.find(v2);
        if parent1 == parent2 {
            return;
        }
        let mut entries = self.entries.borrow_mut();
        let rank1 = entries[parent1].rank;
        let rank2 = entries[parent2].rank;
        if rank1 < rank2 {
            entries[parent1].parent = parent2;
        } else if rank1 > rank2 {
            entries[parent2].parent = parent1;
        } else {
            entries[parent2].parent = parent1;
            entries[parent1].rank += 1;
        }
        self.num_clusters -= 1;
    }

    // Creates a new Union-Find data structure. Note that index 0 is present despite our
    // vertices ranging from 1-n.
    pub fn from(vertices: Vec<Vertex>) -> UnionFind {
        let num_clusters = vertices.len();
        let mut entries = vec![UnionFindEntry { parent: 0, rank: 0 }; num_clusters + 1];
        for vertex in vertices {
            entries[vertex].parent = vertex;
        }
        UnionFind {
            entries: RefCell::new(entries),
            num_clusters,
        }
    }

    // Returns true if both vertices in the given Edge are in different clusters.
    pub fn different_clusters(&self, edge: &Edge) -> bool {
        return self.find(edge.from) != self.find(edge.to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_edgelist() -> EdgeList {
        EdgeList(RefCell::new(vec![
            Edge {
                from: 1,
                to: 2,
                dist: 3,
            },
            Edge {
                from: 1,
                to: 3,
                dist: 4,
            },
            Edge {
                from: 2,
                to: 3,
                dist: 1,
            },
        ]))
    }

    fn setup_unionfind() -> UnionFind {
        UnionFind {
            entries: RefCell::from(vec![
                UnionFindEntry { parent: 0, rank: 0 },
                UnionFindEntry { parent: 1, rank: 0 },
                UnionFindEntry { parent: 2, rank: 0 },
                UnionFindEntry { parent: 3, rank: 0 },
            ]),
            num_clusters: 3,
        }
    }

    #[test]
    fn test_get_vertices() {
        let mut vertices = setup_edgelist().get_vertices();
        vertices.sort();
        assert_eq!(vertices, vec![1, 2, 3])
    }

    #[test]
    fn test_fromfile() {
        assert_eq!(EdgeList::from_file("testfile.txt"), setup_edgelist())
    }

    #[test]
    fn test_clustering() {
        let graph = setup_edgelist();
        assert_eq!(graph.cluster(3), Some(1));
        assert_eq!(graph.cluster(2), Some(3));
        assert_eq!(graph.cluster(1), None);
    }

    #[test]
    fn test_uf_from() {
        let graph = setup_edgelist();
        let uf_test = UnionFind::from(graph.get_vertices());
        let uf = setup_unionfind();
        assert_eq!(uf_test, uf)
    }

    #[test]
    fn test_union_find() {
        let mut uf = setup_unionfind();

        uf.union(1, 2);
        assert_eq!(
            uf,
            UnionFind {
                entries: RefCell::from(vec![
                    UnionFindEntry { parent: 0, rank: 0 },
                    UnionFindEntry { parent: 1, rank: 1 },
                    UnionFindEntry { parent: 1, rank: 0 },
                    UnionFindEntry { parent: 3, rank: 0 },
                ]),
                num_clusters: 2,
            }
        );

        assert_eq!(uf.find(2), 1);

        uf.union(1, 3);
        assert_eq!(
            uf,
            UnionFind {
                entries: RefCell::from(vec![
                    UnionFindEntry { parent: 0, rank: 0 },
                    UnionFindEntry { parent: 1, rank: 1 },
                    UnionFindEntry { parent: 1, rank: 0 },
                    UnionFindEntry { parent: 1, rank: 0 },
                ]),
                num_clusters: 1,
            }
        );
    }
}
