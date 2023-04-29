use std::{collections::HashSet, fs, hash::Hash, str::FromStr, vec};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Code(String);

#[derive(Debug, Eq, PartialEq)]
pub struct CodeList(Vec<Code>);

impl Code {
    pub fn hamming_distance(&self, s: &Code) -> usize {
        self.0
            .chars()
            .zip(s.0.chars())
            .fold(0, |acc, (s1, s2)| if s1 != s2 { acc + 1 } else { acc })
    }

    // Returns true if Hamming distance between self and s is less than spacing. Otherwise false.
    // Short circuits if value is false.
    pub fn distance_less_than(&self, s: &Code, spacing: usize) -> bool {
        let mut different = 0;
        for (s1, s2) in self.0.chars().zip(s.0.chars()) {
            if s1 != s2 {
                different += 1;
                if spacing == different {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct ParseCodeError;

// This is so lazy but I don't feel like improving on it. User beware!
impl FromStr for Code {
    type Err = ParseCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.split_whitespace().collect();
        Ok(Code(code))
    }
}

// By extension, this is lazy too.
impl FromStr for CodeList {
    type Err = ParseCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code_list = s
            .lines()
            .skip(1)
            .map(|s| Code::from_str(s).unwrap())
            .collect();

        Ok(CodeList(code_list))
    }
}

impl CodeList {
    /*
    We must find the maximum number of clusters where the spacing is at least spacing i.e. if two
    strings are Hamming distance < spacing apart they must be in the same cluster. This is just a
    graph search problem, where edges with weight >= spacing are deleted. If two strings are
    connected by some path where each edge has distance <= spacing then they belong in the same
    cluster. We will arbitrarily choose DFS, perhaps we will profile and choose BFS. At the
    moment, only returns the number of clusters.
    */
    pub fn cluster(&self, spacing: usize) -> usize {
        let mut remaining: HashSet<Code> = self.0.iter().cloned().collect();
        let mut clusters = vec![];

        // pop an arbitrary element from remaining. If remaining is non-empty, add all
        // reachable vertices from vertex to the vector of clusters. Else, we're done.
        while let Some(vertex) = remaining.iter().next().cloned() {
            clusters.push(find_cluster(vertex, &mut remaining, spacing))
        }
        clusters.len()
    }

    pub fn from_file(path: &str) -> CodeList {
        let file_data = fs::read_to_string(path).unwrap();
        CodeList::from_str(&file_data).unwrap()
    }
}

// Finds all vertices in the same cluster as the given vertex for the given spacing i.e.
// a vertex belongs in this cluster if the Hamming distance between it and another vertex in the
// cluster is < spacing. Returns a vector of all vertices in the cluster and removes these vertices
// from remaining.
fn find_cluster(vertex: Code, remaining: &mut HashSet<Code>, spacing: usize) -> Vec<Code> {
    remaining.remove(&vertex);
    let mut todo = vec![vertex];
    let mut reachable = todo.clone();

    while let Some(vertex) = todo.pop() {
        process_neighbors(vertex, &mut reachable, remaining, &mut todo, spacing);
    }
    reachable
}

// Finds all reachable vertices from vertex, adds them to reachable and todo, and removes them from
// remainining.
fn process_neighbors(
    vertex: Code,
    reachable: &mut Vec<Code>,
    remaining: &mut HashSet<Code>,
    todo: &mut Vec<Code>,
    spacing: usize,
) {
    let neighbors = remaining
        .iter()
        .filter(|&s| vertex.distance_less_than(s, spacing))
        .cloned()
        .collect::<Vec<Code>>();
    todo.extend_from_slice(&neighbors[..]);
    reachable.extend_from_slice(&neighbors[..]);

    for neighbor in neighbors {
        remaining.remove(&neighbor);
    }
}

#[cfg(test)]
mod tests {
    use super::{Code, CodeList};
    #[test]
    fn test_hamming_distance() {
        let s1 = Code(String::from("1010"));
        let s2 = Code(String::from("1100"));
        let s3 = Code(String::from("1011"));
        assert_eq!(s1.hamming_distance(&s2), 2);
        assert_eq!(s1.hamming_distance(&s3), 1);
        assert_eq!(s3.hamming_distance(&s2), 3);

        assert!(s1.distance_less_than(&s2, 3));
        assert!(!s1.distance_less_than(&s2, 2));
        assert!(s3.distance_less_than(&s3, 1));
    }

    #[test]
    fn test_clustering() {
        let s1 = Code(String::from("1010"));
        let s2 = Code(String::from("1100"));
        let s3 = Code(String::from("1011"));

        let code_list = CodeList(vec![s1, s2, s3]);

        assert_eq!(code_list.cluster(1), 3);
        assert_eq!(code_list.cluster(2), 2);
        assert_eq!(code_list.cluster(3), 1);
    }
}
