use std::{collections::BinaryHeap, ops::Add};
type Value = usize;
type Weight = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Item {
    value: Value,
    weight: Weight,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Knapsack {
    pub size: Weight,
    pub items: Vec<Item>,
}

// reverse ord because heap is max heap by default, and we want lowest weight items to surface up
// first.
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            value: 0,
            weight: 0,
        }
    }
}

impl Add for Item {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Item {
            weight: self.weight + rhs.weight,
            value: self.value + rhs.value,
        }
    }
}

impl Item {
    pub fn from_str(data: &str) -> Self {
        let item = data
            .split(&" ")
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()
            .unwrap();
        Self {
            value: item[0],
            weight: item[1],
        }
    }

    pub fn from(value: usize, weight: usize) -> Self {
        Self { value, weight }
    }
}

impl Knapsack {
    pub fn from(data: &str) -> Self {
        let mut lines = data.lines();
        let size = lines.next().unwrap().parse().unwrap();
        let items = lines.map(Item::from_str).collect();
        Self { size, items }
    }

    pub fn max_value(&self) -> usize {
        let n = self.items.len();
        // results[i] = (total value, weight used)
        let mut results = vec![vec![]; n + 1];
        results[0].push(Item {
            weight: 0,
            value: 0,
        });
        for i in (1..=n) {
            let item = self.items[i];
        }
        todo!()
    }

    // creates the array of values that contain the total budget used and the values of the
    // knapsack with various items in it.
    fn create_knapsack_array(&self) -> Vec<BinaryHeap<Item>> {
        let n = self.items.len();
        let mut array = vec![BinaryHeap::new(); n];
        array[0].push(Item::default());
        array[0].push(self.items[0]);
        for i in 1..n {
            array[i] = array[i - 1].clone();
            let mut todo = array[i - 1].clone();
            let current = self.items[i];
            while let Some(item) = todo.pop() {
                if current.weight + item.weight <= self.size {
                    array[i].push(current + item)
                } else {
                    break;
                }
            }
        }
        array
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use super::{Item, Knapsack};

    fn create_knapsack() -> Knapsack {
        Knapsack {
            size: 10,
            items: vec![
                Item {
                    value: 10,
                    weight: 5,
                },
                Item {
                    value: 3,
                    weight: 6,
                },
                Item {
                    value: 7,
                    weight: 4,
                },
            ],
        }
    }

    #[test]
    fn test_array() {
        let sack = create_knapsack();
        let sack_array = vec![
            vec![Item::from(10, 5), Item::default()],
            vec![Item::from(3, 6), Item::from(10, 5), Item::default()],
            vec![
                Item::from(10, 10),
                Item::from(17, 9),
                Item::from(3, 6),
                Item::from(10, 5),
                Item::from(7, 4),
                Item::default(),
            ],
        ];
        let my_impl = sack
            .create_knapsack_array()
            .into_iter()
            .map(BinaryHeap::into_sorted_vec)
            .collect::<Vec<Vec<_>>>();

        assert_eq!(my_impl, sack_array)
    }

    #[test]
    fn test_max_value() {
        let sack = create_knapsack();
        assert_eq!(sack.max_value(), 10)
    }
}
