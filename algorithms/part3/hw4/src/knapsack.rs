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
}

impl Knapsack {
    pub fn from_str(data: &str) -> Self {
        let mut lines = data.lines();
        let size = lines
            .next()
            .unwrap()
            .split(&" ")
            .take(1)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()[0];
        let items = lines.map(Item::from_str).collect();
        Self { size, items }
    }

    pub fn max_value(&self) -> Value {
        let max_weight = self.size;
        // at each iteration through the items of the knapsack, current_array[i] is the highest
        // value knapsack with weight up to and including i.
        let mut last_array = vec![0; max_weight + 1];
        //let mut i = 0;
        for item in &self.items {
            let mut current_array = last_array.clone();
            //dbg!(i);
            //i += 1;
            for i in item.weight..=max_weight {
                current_array[i] = last_array[i].max(last_array[i - item.weight] + item.value)
            }
            //dbg!(&current_array);
            last_array = current_array;
        }
        *last_array.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
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
    fn test_max_value() {
        let sack = create_knapsack();
        assert_eq!(sack.max_value(), 17)
    }
}
