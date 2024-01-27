type Coord = f32;
type CityID = usize;
type SubsetID = usize;

// City coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
struct City(Coord, Coord);

// City container
#[derive(Debug)]
pub struct Cities(Vec<City>);

// Denotes membership in a subset bitwise. i.e. if item 1 is contained in the subset but nothing
// else is the value of the usize is 0b10
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Subset(SubsetID);

// Contains a Vec of CityID's and the corresponding subset representation
#[derive(PartialEq, Eq, Debug)]
struct CitySubset {
    ids: Vec<CityID>,
    subset: Subset,
}

// Used to iterate over all subsets of size subset_size out of n possible members. Subsets are
// returned as a Vec of indices.
// Generators in rust are kinda awkward, I would need another struct in order to avoid cloning
// the interior state. maybe fix in future? (prob not)
#[derive(Debug)]
struct SubsetIterator {
    n: usize,
    ids: Vec<CityID>,
    finished: bool,
}

impl City {
    // Constructs a City from the given &str
    fn from_str(data: &str) -> Self {
        let (x, y) = data.split_once(" ").unwrap();
        City(x.parse().unwrap(), y.parse().unwrap())
    }

    // Computes the Euclidean distance between two Cities
    fn dist(&self, other: &Self) -> Coord {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

impl Cities {
    // Constructs a Cities struct from the given &str
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
        for CitySubset { ids, subset } in SubsetIterator::all_subsets(n, 1) {
            dp_array[subset.id()][ids[0]] = source.dist(&cities[ids[0]]);
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

    // Computes the shortest tour of all the Cities i.e. solves the Traveling Salesman Problem
    pub fn tsp2(&self) -> Coord {
        // Arbitrarily choose first city as starting point.
        let source = &self.0[0];
        let cities = &self.0[1..];
        let n = cities.len();
        let mut dp_array = vec![vec![Coord::INFINITY; n]; (2_usize).pow(n as u32)];
        // base case
        SubsetIterator::all_subsets(n, 1).for_each(|CitySubset { ids, subset }| {
            dp_array[subset.id()][ids[0]] = source.dist(&cities[ids[0]])
        });
        // Iterate over subset sizes
        (2..=n).into_iter().for_each(|subset_size| {
            SubsetIterator::all_subsets(n, subset_size)
                .for_each(|city_sub| city_sub.update_array2(&mut dp_array, cities))
        });

        let all_cities = CitySubset::from_ids(&(0..n).collect());
        all_cities.min_dist_to2(
            &source,
            all_cities.all_ids(),
            &dp_array[all_cities.subset.id()],
            &cities,
        )
    }
}

impl Subset {
    // Turns list of city id's into corresponding usize representation.
    /// assert_eq!(Subset(0b101), Subset::from_ids(vec![0, 2]))
    fn from_ids(ids: &[CityID]) -> Self {
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
    fn remove(&self, id: CityID) -> Self {
        Subset(self.0 ^ (1 << id))
    }

    fn id(&self) -> SubsetID {
        self.0
    }
}

impl CitySubset {
    // Takes a Vec of CityID's and returns a CitySubset corresponding to those ID's
    fn from_ids(ids: &Vec<CityID>) -> Self {
        Self {
            ids: ids.clone(),
            subset: Subset::from_ids(ids),
        }
    }

    // Returns an iterator over all CityID's in the CitySubset.
    fn all_ids(&self) -> impl Iterator<Item = &CityID> {
        self.ids.iter()
    }

    // Returns an iterator over all CityID's in the CitySubset except for the given CityID.
    fn all_ids_except(&self, id: CityID) -> impl Iterator<Item = &CityID> {
        self.all_ids().filter(move |&&other| other != id)
    }

    // Computes the min distance to the given CityID that visits each City in the CitySubset
    // exactly once.
    fn min_dist_to(&self, id: usize, dp_array: &mut [Vec<Coord>], cities: &[City]) -> Coord {
        let dest = cities[id];
        let prev_sub = self.subset.remove(id).id();
        self.all_ids_except(id)
            .fold(Coord::INFINITY, |accum, &other_id| {
                accum.min(dp_array[prev_sub][other_id] + dest.dist(&cities[other_id]))
            })
    }

    // Computes the distance to to from all CityID's returned by the iterator from
    fn min_dist_to2<'a, I>(
        &self,
        to: &City,
        from: I,
        distances: &Vec<Coord>,
        cities: &[City],
    ) -> Coord
    where
        I: Iterator<Item = &'a CityID>,
    {
        from.fold(Coord::INFINITY, |accum, id| {
            accum.min(distances[*id] + cities[*id].dist(&to))
        })
    }

    // For each city in the CitySubset, computes the shortest path that ends at the given city
    // visiting each city in the subset exactly once.
    fn update_array(&self, dp_array: &mut [Vec<Coord>], cities: &[City]) {
        let sub_id = self.subset.id();
        self.all_ids()
            .for_each(|&id| dp_array[sub_id][id] = self.min_dist_to(id, dp_array, cities))
    }

    // For each city in the CitySubset, computes the shortest path that ends at the given city
    // visiting each city in the subset exactly once.
    fn update_array2(&self, dp_array: &mut [Vec<Coord>], cities: &[City]) {
        let sub = self.subset;
        self.all_ids().for_each(|&id| {
            dp_array[sub.id()][id] = self.min_dist_to2(
                &cities[id],
                self.all_ids_except(id),
                &dp_array[sub.remove(id).id()],
                cities,
            )
        });
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
