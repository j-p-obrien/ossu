use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Vertex = u64;
type Cost = u64;

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    from: Vertex,
    to: Vertex,
    cost: Cost,
}

// An EdgeList. The edges are sorted by ascending edge cost.
#[derive(Debug, PartialEq, Eq)]
pub struct EdgeList(Vec<Edge>);

pub struct UnionFind<T>(HashMap<T, T>);

impl EdgeList {
    // Computes max-spacing k-clustering using Kruskal's MST algorithm. Returns something,
    // idk yet haven't decided.
    pub fn cluster(&self, k: u32) {
        let vertices = self.get_vertices();
        let mut num_clusters = vertices.len();
        let mut vertices = UnionFind::from(vertices);

        for Edge { from, to, .. } in &self.0 {
            let no_cycle = vertices.leader(from) != vertices.leader(to);
            if no_cycle {
                vertices.union(from, to);
                num_clusters -= 1;
                if num_clusters == k as usize {
                    break;
                }
            }
        }
        todo!()
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        todo!()
    }
}

impl<T> UnionFind<T> {
    pub fn leader(&self, vertex: &T) -> T {
        todo!()
    }

    pub fn union(&mut self, vertex1: &T, vertex2: &T) {
        todo!()
    }

    pub fn from(vertices: Vec<T>) -> UnionFind<T> {
        todo!()
    }
}
