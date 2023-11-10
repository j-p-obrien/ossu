use std::collections::hash_map;

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    value: usize,
    weight: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Knapsack {
    pub size: usize,
    pub items: Vec<Item>,
}

impl Item {
    pub fn from(data: &str) -> Self {
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
}

impl Knapsack {
    pub fn from(data: &str) -> Self {
        let mut lines = data.lines();
        let size = lines.next().unwrap().parse().unwrap();
        let items = lines.map(Item::from).collect();
        Self { size, items }
    }

    pub fn max_value(&self) -> usize {
        let n = self.items.len();
        let mut results = vec![vec![]; n];
        for i in (0..n).rev() {}
        todo!()
    }
}

#[cfg(test)]
mod tests {}
