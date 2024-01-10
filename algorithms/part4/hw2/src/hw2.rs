type Coord = f32;

// City coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
struct City(Coord, Coord);

// City container
#[derive(Debug)]
pub struct Cities(Vec<City>);

// Denotes membership in a subset using bitwise operations. i.e. if item 1 is contained in the
// subset but nothing else is the value of the usize is 0b10
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Subset(usize);

#[derive(Debug, Clone, Copy)]
struct IterState {
    current: usize,
    end: usize,
}

// Used to iterate over all subsets of size subset_size out of n total possible members.
// We will denote membership bitwise, where a city i in Cities corresponds to bit i in current.
// We will use the value of current as an index in a DP array to solve the TSP.
#[derive(Debug)]
struct SubsetIterator {
    n: usize,
    state: Vec<IterState>,
}

impl City {
    fn from_str(data: &str) -> Self {
        let (x, y) = data.split_once(" ").unwrap();
        City(x.parse().unwrap(), y.parse().unwrap())
    }

    fn dist(&self, other: &Self) -> Coord {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

impl Cities {
    pub fn from_str(data: &str) -> Self {
        Cities(data.lines().skip(1).map(City::from_str).collect())
    }

    // Computes the shortest tour of all the Cities i.e. solves the Traveling Salesman Problem
    pub fn tsp(&self) -> f32 {
        // Arbitrarily choose last city as starting point.
        let mut cities = self.0.clone();
        let source = cities.pop().unwrap();
        let n = cities.len();
        // Iterate over subset sizes
        for m in 0..n {}
        todo!()
    }
}

impl Subset {
    fn from_indices(indices: &[usize]) -> Self {
        let mut subset = 0;
        for i in indices {
            subset |= 1 << *i
        }
        Self(subset)
    }
}

// end[i] = n - subset_size + i
impl SubsetIterator {
    fn all_subsets(n: usize, subset_size: usize) -> Self {
        assert!(subset_size <= n);
        let state = (0..subset_size)
            .map(|i| IterState {
                current: i,
                end: n - subset_size + i,
            })
            .collect();
        Self { n, state }
    }

    fn next_state(&mut self, i: isize) {
        //hacky lol
        if i < 0 {
            self.state[0].current += 1;
            return ();
        }
        let index = i as usize;
        self.state[index].current += 1;
        for i in (index + 1)..self.state.len() {
            self.state[i].current = self.state[i - 1].current + 1;
        }
    }

    fn iter_return(&self) -> (Subset, Vec<usize>) {
        let indices: Vec<_> = self.state.iter().map(|s| s.current).collect();
        (Subset::from_indices(&indices), indices)
    }
}

// Iterate over all subsets
impl Iterator for SubsetIterator {
    type Item = (Subset, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.state[0].current > self.state[0].end {
            return None;
        }
        let return_val = Some(self.iter_return());
        // Find next member that hasn't rolled over
        let mut i = self.state.len() as isize - 1;
        while i >= 0 {
            if self.state[i as usize].current == self.state[i as usize].end {
                i -= 1;
            } else {
                break;
            }
        }
        self.next_state(i);
        return_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_iteration1() {
        let (len, subset_size) = (3, 1);
        let mut sub_iter = SubsetIterator::all_subsets(len, subset_size);
        assert_eq!(sub_iter.next(), Some((Subset(0b_001), vec![0])));
        assert_eq!(sub_iter.next(), Some((Subset(0b_010), vec![1])));
        assert_eq!(sub_iter.next(), Some((Subset(0b_100), vec![2])));
        assert_eq!(sub_iter.next(), None);
    }

    #[test]
    fn test_subset_iteration2() {
        let (len, subset_size) = (4, 2);
        let mut sub_iter = SubsetIterator::all_subsets(len, subset_size);
        assert_eq!(sub_iter.next(), Some((Subset(0b_0011), vec![0, 1])));
        assert_eq!(sub_iter.next(), Some((Subset(0b_0101), vec![0, 2])));
        assert_eq!(sub_iter.next(), Some((Subset(0b_1001), vec![0, 3])));
        assert_eq!(sub_iter.next(), Some((Subset(0b_0110), vec![1, 2])));
        assert_eq!(sub_iter.next(), Some((Subset(0b_1010), vec![1, 3])));
        assert_eq!(sub_iter.next(), Some((Subset(0b_1100), vec![2, 3])));
        assert_eq!(sub_iter.next(), None);
    }
}
