use std::{
    collections::{HashSet, VecDeque},
    fs,
    hash::Hash,
    str::FromStr,
    vec,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Code {
    string: String,
    sum: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CodeList(Vec<Code>);

impl Code {
    pub fn hamming_distance(&self, s: &Code) -> usize {
        self.string
            .chars()
            .zip(s.string.chars())
            .fold(0, |acc, (s1, s2)| if s1 != s2 { acc + 1 } else { acc })
    }

    // Returns true if Hamming distance between self and s is less than spacing. Otherwise false.
    // Short circuits if value is false.
    pub fn distance_less_than(&self, s: &Code, spacing: usize) -> bool {
        let mut different = 0;
        for (s1, s2) in self.string.chars().zip(s.string.chars()) {
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

#[derive(Debug, Eq, PartialEq)]
pub struct ParseCodeError;

// This is so lazy but I don't feel like improving on it. User beware!
impl FromStr for Code {
    type Err = ParseCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: String = s.split_whitespace().collect();
        let sum = string.matches("1").count();
        Ok(Code { string, sum })
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
    connected by some path where each edge has distance < spacing then they belong in the same
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

    pub fn cluster_optimized(&self, spacing: usize) -> usize {
        let mut buckets = bucket_codes(&self.0);
        let mut clusters = vec![];
        for i in 0..buckets.len() {
            while let Some(code) = buckets[i].pop() {
                clusters.push(find_all_neighbors(code, &mut buckets, spacing));
            }
            //dbg!(clusters.len());
        }
        clusters.len()
    }
}

fn find_all_neighbors(code: Code, buckets: &mut Vec<Vec<Code>>, spacing: usize) -> Vec<Code> {
    let mut neighbors = vec![code.clone()];
    let mut todo = VecDeque::from([code]);
    while let Some(code) = todo.pop_front() {
        //let mut todo = vec![code];
        //while let Some(code) = todo.pop() {
        let sum = code.sum;
        for bucket_num in (sum - 2)..=(sum + 2) {
            let bucket = buckets.get_mut(bucket_num);
            let next = take_neighbors_from_bucket(&code, bucket, spacing);
            neighbors.extend_from_slice(&next);
            //dbg!(neighbors.len());
            todo.extend(next.into_iter());
        }
    }
    neighbors
}

fn take_neighbors_from_bucket(
    code: &Code,
    bucket: Option<&mut Vec<Code>>,
    spacing: usize,
) -> Vec<Code> {
    if let Some(bucket) = bucket {
        let (indices, _): (Vec<usize>, Vec<_>) = bucket
            .iter()
            .enumerate()
            .filter(|(_, s)| code.distance_less_than(s, spacing))
            .unzip();
        return remove_from_bucket(indices, bucket);
    } else {
        return vec![];
    }
}

fn remove_from_bucket(indices: Vec<usize>, bucket: &mut Vec<Code>) -> Vec<Code> {
    let mut neighbors = Vec::with_capacity(indices.len());
    // We must reverse indices here since indices are in ascending order. Consider the case where
    // an item we want to remove is at the end of the vector. We would swap it and lose its place.
    for &i in indices.iter().rev() {
        neighbors.push(bucket.swap_remove(i))
    }
    neighbors
}

// buckets codes according to their sum. If two strings have Hamming distance < k then their
// sum differs by less than k as well.
fn bucket_codes(codes: &Vec<Code>) -> Vec<Vec<Code>> {
    let max_sum = codes
        .iter()
        .fold(0, |acc, code| if code.sum > acc { code.sum } else { acc });
    let mut buckets = vec![vec![]; max_sum + 1];
    for code in codes {
        buckets[code.sum].push(code.clone())
    }
    buckets
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
    dbg!(reachable.len());
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
    dbg!(remaining.capacity());
    remaining.shrink_to_fit();
    dbg!(remaining.capacity());
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Code, CodeList};

    fn setup_codelist() -> CodeList {
        CodeList(vec![
            Code::from_str("1010").unwrap(),
            Code::from_str("1100").unwrap(),
            Code::from_str("1011").unwrap(),
        ])
    }

    #[test]
    fn test_from_str() {
        let s1 = Code::from_str("0 1 0 1");
        let s2 = Code::from_str("1 1 0 1");
        assert_eq!(
            s1,
            Ok(Code {
                string: String::from("0101"),
                sum: 2
            })
        );
        assert_eq!(
            s2,
            Ok(Code {
                string: String::from("1101"),
                sum: 3
            })
        );
    }

    #[test]
    fn test_from_file() {
        let codes = CodeList::from_file("hamming_testfile.txt");
        assert_eq!(codes, setup_codelist())
    }

    #[test]
    fn test_hamming_distance() {
        if let [s1, s2, s3] = &setup_codelist().0[..] {
            assert_eq!(s1.hamming_distance(s2), 2);
            assert_eq!(s1.hamming_distance(s3), 1);
            assert_eq!(s3.hamming_distance(s2), 3);

            assert!(s1.distance_less_than(s2, 3));
            assert!(!s1.distance_less_than(s2, 2));
            assert!(s3.distance_less_than(s3, 1));
        } else {
            panic!("Something went wrong")
        }
    }

    #[test]
    fn test_clustering() {
        let code_list = setup_codelist();

        assert_eq!(code_list.cluster_optimized(1), 3);
        assert_eq!(code_list.cluster_optimized(2), 2);
        assert_eq!(code_list.cluster_optimized(3), 1);
    }
}
