pub struct MWIS {
    pub weights: Vec<usize>,
}

impl MWIS {
    // computes the maximum weight independent set of the vertices in the path graph self
    // returns a vector results where results[i] is true if vertex i is in the maximum weight
    // independent set.
    pub fn mwis(&self) -> Vec<bool> {
        let n = self.weights.len();
        let mut included = vec![false; n];
        let mut max_array = vec![0; n];
        max_array[0] = self.weights[0];
        max_array[1] = self.weights[1];
        for vertex in 2usize..n {
            max_array[vertex] =
                max_array[vertex - 1].max(max_array[vertex] + max_array[vertex - 2]);
        }

        // reconstruct
        for vertex in (1..n).rev() {
            included[vertex] = max_array[vertex] > max_array[vertex - 1];
        }
        included[0] = !included[1];

        included
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mwis() {}
}
