use std::{collections::BinaryHeap, cmp::Reverse};

pub trait MedianMaintainer<T>
where T: Ord 
{
    fn push(&mut self, val: T);
    fn peek(&self) -> Option<&T>;
}

#[derive(Debug)]
pub struct HeapMM<T> {
    lower: BinaryHeap<T>,
    upper: BinaryHeap<Reverse<T>>
}

impl<T> MedianMaintainer<T> for HeapMM<T> 
where T: Ord
{
    fn push(&mut self, val: T) {
        if let Some(median) = self.lower.peek() {
            if val <= *median {
                self.lower.push(val);
            }
            else {
                self.upper.push(Reverse(val));
            }
            self.rebalance()
        }
        else {
            self.lower.push(val)
        }
    }

    fn peek(&self) -> Option<&T> {
        self.lower.peek()
    }
}

impl<T> HeapMM<T> 
where T: Ord 
{
    fn rebalance(&mut self) {
        if self.lower.len() > self.upper.len() + 1 {
            if let Some(val) = self.lower.pop() {
                self.upper.push(Reverse(val))
            }
        }
        else if self.upper.len() > self.lower.len() {
            if let Some(Reverse(val)) = self.upper.pop() {
                self.lower.push(val)
            }
        }
    }

    pub fn new() -> HeapMM<T> {
        HeapMM { lower: BinaryHeap::new(), upper: BinaryHeap::new() }
    }

}

#[cfg(test)]
mod tests {
    use crate::{HeapMM, MedianMaintainer};


    #[test]
    fn test_hmm() {
        let mut hmm: HeapMM<i32> = HeapMM::new();
        assert_eq!(hmm.peek(), None);

        hmm.push(3);
        assert_eq!(hmm.peek(), Some(&3));

        hmm.push(2);
        assert_eq!(hmm.peek(), Some(&2));

        hmm.push(10);
        assert_eq!(hmm.peek(), Some(&3))
    }

}

