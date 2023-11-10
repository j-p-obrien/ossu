#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    value: usize,
    weight: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Knapsack {
    pub size: usize,
    pub weights: Vec<Item>,
}

impl Item {
    pub fn from(data: &str) -> Self {
        todo!()
    }
}

impl Knapsack {
    pub fn from(data: &str) -> Self {
        todo!()
    }

    pub fn max_value(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
