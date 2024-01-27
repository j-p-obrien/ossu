type Coord = f32;
type CityID = usize;
type SubsetID = usize;

// City coordinates
struct City(Coord, Coord);

// City container
pub struct Cities(Vec<City>);

// Denotes membership in a subset bitwise. i.e. if item 1 is contained in the subset but nothing
// else is the value of the usize is 0b10
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Subset(SubsetID);

// Contains a Vec of CityID's and the corresponding subset representation. We could get away
// with either/or but do it this way for performance reasons
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

// Holds the results for the DP algorithm. First slot is indexed with a Subset. Second is indexed
// by a CityID in the subset. Values are the minimum path length ending in the specified CityID,
// with each city in the subset visited exactly once.
struct ResultsArray(Vec<Vec<Coord>>);

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
        let mut dp_array = ResultsArray::init(n);
        // base case
        SubsetIterator::all_subsets(n, 1).for_each(|CitySubset { ids, subset }| {
            dp_array.0[subset.id()][ids[0]] = source.dist(&cities[ids[0]])
        });
        // Iterate over subset sizes
        (2..=n).into_iter().for_each(|subset_size| {
            SubsetIterator::all_subsets(n, subset_size)
                .for_each(|city_sub| dp_array.update_array(city_sub, &cities))
        });

        let all_cities = CitySubset::from_ids(&(0..n).collect());
        dp_array.min_dist_to(&source, all_cities.all_ids(), all_cities.subset, &cities)
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

impl ResultsArray {
    fn init(n: usize) -> Self {
        Self(vec![vec![Coord::INFINITY; n]; (2_usize).pow(n as u32)])
    }

    fn update_array(&mut self, city_sub: CitySubset, cities: &[City]) {
        let sub = city_sub.subset;
        city_sub.all_ids().for_each(|&id| {
            self.0[city_sub.subset.id()][id] = self.min_dist_to(
                &cities[id],
                city_sub.all_ids_except(id),
                sub.remove(id),
                cities,
            )
        });
    }

    fn min_dist_to<'a, I>(&self, to: &City, from: I, city_sub: Subset, cities: &[City]) -> f32
    where
        I: Iterator<Item = &'a CityID>,
    {
        from.fold(Coord::INFINITY, |accum, &id| {
            accum.min(self.0[city_sub.id()][id] + cities[id].dist(to))
        })
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
