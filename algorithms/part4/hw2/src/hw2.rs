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

#[derive(PartialEq, Eq, Debug)]
struct CitySubset {
    ids: Vec<usize>,
    subset: Subset,
}

// Used to iterate over all subsets of size subset_size out of n possible members. Subsets are
// returned as a Vec of indices.
// Generators in rust are kinda awkward, I would need another struct in order to avoid cloning
// the interior state. maybe fix in future? (prob not)
#[derive(Debug)]
struct SubsetIterator {
    n: usize,
    ids: Vec<usize>,
    finished: bool,
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
    pub fn tsp(&self) -> Coord {
        // Arbitrarily choose first city as starting point.
        let source = &self.0[0];
        let cities = &self.0[1..];
        let n = cities.len();
        let mut dp_array = vec![vec![Coord::INFINITY; n]; (2_usize).pow(n as u32)];
        // base case
        for city_sub in SubsetIterator::all_subsets(n, 1) {
            dp_array[city_sub.subset.id()][city_sub.ids[0]] = source.dist(&cities[city_sub.ids[0]]);
        }
        // Iterate over subset sizes
        for subset_size in 2..=n {
            for city_sub in SubsetIterator::all_subsets(n, subset_size) {
                city_sub.update_array(&mut dp_array, cities)
            }
        }
        let mut smallest_dist = Coord::INFINITY;
        for CitySubset { ids, subset } in SubsetIterator::all_subsets(n, n) {
            for id in ids {
                smallest_dist =
                    smallest_dist.min(dp_array[subset.id()][id] + source.dist(&cities[id]))
            }
        }
        smallest_dist
    }
}

impl Subset {
    // Turns list of city id's into corresponding usize representation.
    /// assert_eq!(Subset(0b101), Subset::from_ids(vec![0, 2]))
    fn from_ids(ids: &[usize]) -> Self {
        let mut subset = 0;
        for &id in ids {
            subset |= 1 << id
        }
        Self(subset)
    }

    // Technically this just changes membership but it is ok for our purposes, I promise to be
    // responsible.
    // We could have also done:
    // Subset(self.0 & !(1 << id))
    fn remove(&self, id: usize) -> Self {
        Subset(self.0 ^ (1 << id))
    }

    fn id(&self) -> usize {
        self.0
    }
}

impl CitySubset {
    fn from_ids(ids: &Vec<usize>) -> Self {
        Self {
            ids: ids.clone(),
            subset: Subset::from_ids(ids),
        }
    }

    fn update_array(&self, dp_array: &mut [Vec<Coord>], cities: &[City]) {
        let current = self.subset;
        for &id in &self.ids {
            let city = cities[id];
            let previous = current.remove(id).id();
            dp_array[current.id()][id] = self.ids.iter().filter(|&&other_id| other_id != id).fold(
                Coord::INFINITY,
                |accum, &other_id| {
                    accum.min(dp_array[previous][other_id] + city.dist(&cities[other_id]))
                },
            );
        }
    }
}

// end[i] = n - subset_size + i
impl SubsetIterator {
    fn all_subsets(n: usize, subset_size: usize) -> Self {
        assert!(subset_size <= n);
        let ids = (0..subset_size).collect();
        Self {
            n,
            ids,
            finished: false,
        }
    }

    // Increments member at given index and resets all members after the given index
    fn increment(&mut self, i: usize) {
        self.ids[i] += 1;
        for j in (i + 1)..self.ids.len() {
            self.ids[j] = self.ids[j - 1] + 1
        }
    }

    // Returns true if the member given by index i is not in its final position.
    fn is_incrementable(&self, i: usize) -> bool {
        self.ids[i] < self.n - self.ids.len() + i
    }
}

// Iterate over all subsets
impl Iterator for SubsetIterator {
    type Item = CitySubset;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let return_val = Some(CitySubset::from_ids(&self.ids));
        // Find next member that hasn't rolled over
        match (0..self.ids.len())
            .rev()
            .find(|&i| self.is_incrementable(i))
        {
            Some(i) => self.increment(i),
            None => self.finished = true,
        };
        return_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_iteration1() {
        let (n, subset_size) = (3, 1);
        let mut sub_iter = SubsetIterator::all_subsets(n, subset_size);
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![0])));
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![1])));
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![2])));
        assert_eq!(sub_iter.next(), None);
    }

    #[test]
    fn test_subset_iteration2() {
        let (n, subset_size) = (4, 2);
        let mut sub_iter = SubsetIterator::all_subsets(n, subset_size);
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![0, 1])));
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![0, 2])));
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![0, 3])));
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![1, 2])));
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![1, 3])));
        assert_eq!(sub_iter.next(), Some(CitySubset::from_ids(&vec![2, 3])));
        assert_eq!(sub_iter.next(), None);
    }

    #[test]
    fn test_subset_iteration3() {
        let (n, subset_size) = (5, 5);
        let mut sub_iter = SubsetIterator::all_subsets(n, subset_size);
        assert_eq!(
            sub_iter.next(),
            Some(CitySubset::from_ids(&vec![0, 1, 2, 3, 4]))
        );
        assert_eq!(sub_iter.next(), None);
    }
}
