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
                max_array[vertex - 1].max(self.weights[vertex] + max_array[vertex - 2]);
        }

        // reconstruct
        let mut vertex: isize = n as isize - 1;
        while vertex > 0 {
            if max_array[vertex as usize] > max_array[vertex as usize - 1] {
                included[vertex as usize] = true;
                vertex -= 2;
            } else {
                vertex -= 1;
            }
        }
        included[0] = vertex == 0;

        included
    }
}

#[cfg(test)]
mod tests {
    use super::MWIS;

    #[test]
    fn test_mwis1() {
        let vertices = MWIS {
            weights: vec![1, 2, 3, 4],
        };

        assert_eq!(vertices.mwis(), vec![false, true, false, true]);
    }

    #[test]
    fn test_mwis2() {
        let vertices = MWIS {
            weights: vec![1, 5, 2, 3, 4],
        };

        assert_eq!(vertices.mwis(), vec![false, true, false, false, true]);
    }
}
