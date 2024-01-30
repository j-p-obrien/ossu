use std::cmp::Ordering;

type Coord = f64;
type CityID = usize;

#[derive(Clone, Copy)]
pub struct City {
    id: CityID,
    x: Coord,
    y: Coord,
}

#[derive(Clone)]
pub struct Cities(Vec<City>);

impl City {
    fn from_str(data: &str) -> Self {
        let (id, coords) = data.split_once(" ").unwrap();
        let (x, y) = coords.split_once(" ").unwrap();
        Self {
            id: id.parse::<usize>().unwrap() - 1,
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn sq_dist(&self, other: &City) -> Coord {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    fn dist(&self, other: &City) -> Coord {
        self.sq_dist(&other).sqrt()
    }

    fn closer(&self, lhs: &City, rhs: &City) -> Ordering {
        if self.sq_dist(lhs) < self.sq_dist(rhs) {
            Ordering::Less
        } else if self.sq_dist(lhs) > self.sq_dist(rhs) {
            Ordering::Greater
        } else {
            lhs.id.cmp(&rhs.id)
        }
    }
}

impl Cities {
    pub fn from_str(data: &str) -> Self {
        Self(data.lines().skip(1).map(City::from_str).collect())
    }

    pub fn greedy_tour(&self) -> Coord {
        let mut todo = self.clone();
        let mut current = todo.0.swap_remove(0);
        let mut tour = vec![current];
        while todo.0.len() > 0 {
            current = todo.remove_closest_city(&current);
            tour.push(current);
        }
        Cities(tour).total_distance()
    }

    fn remove_closest_city(&mut self, current: &City) -> City {
        let (i, _) = self
            .0
            .iter()
            .enumerate()
            .min_by(|(_, lhs), (_, rhs)| current.closer(lhs, rhs))
            .unwrap();
        self.0.swap_remove(i)
    }

    fn total_distance(&self) -> Coord {
        self.0
            .windows(2)
            .fold(0.0, |accum, cities| accum + cities[0].dist(&cities[1]))
            + self.0[0].dist(self.0.last().unwrap())
    }
}
