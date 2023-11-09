use std::collections::BinaryHeap;

// Tree containing CodeNodes. Since we only care about the length of the codewords, we only need
// to store the internal nodes of the encoding tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HuffmanEncoding {
    codes: Vec<CodeNode>,
}

// Internal node of the Huffman Encoding tree. We leaf out leaf nodes, which are simply the
// original codewords.
// left: index of left child in HuffmanEncoding vector
// right: index of right child in HuffmanEncoding vector
// If left/right is None, then that child is an original codeword. If it is Some(index),
// then index is the index in HuffmanEncoding.codes.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CodeNode {
    left: Option<usize>,
    right: Option<usize>,
}

// Keeps track of the weights of the original and combined codewords. id is None if it is an
// original codeword. id is Some(id) if it corresponds to an internal node on the Huffman tree.  If
// this is the case, id is the index in HuffmanEncoding.codes
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct HeapCode {
    id: Option<usize>,
    weight: usize,
}

impl PartialOrd for HeapCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl Ord for HeapCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl HuffmanEncoding {
    pub fn encode(weights: Vec<usize>) -> Self {
        let mut heap: BinaryHeap<_> = weights
            .iter()
            .map(|&w| HeapCode {
                id: None,
                weight: w,
            })
            .collect();

        let mut codes = Vec::with_capacity(heap.len() - 1);
        let mut id = 0;
        while heap.len() > 1 {
            let code1 = heap.pop().unwrap();
            let code2 = heap.pop().unwrap();
            codes.push({
                CodeNode {
                    left: code1.id,
                    right: code2.id,
                }
            });
            heap.push(HeapCode {
                id: Some(id),
                weight: code1.weight + code2.weight,
            });
            id += 1;
        }

        HuffmanEncoding { codes }
    }

    // Returns the minimum and maximum length of the codewords, respectively
    pub fn min_max_length(&self) -> (usize, usize) {
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        let mut todo = vec![(self.codes[self.codes.len() - 1], 1)];
        while let Some((current, depth)) = todo.pop() {
            match (current.left, current.right) {
                // Both children are leaf nodes
                (None, None) => {
                    max = max.max(depth);
                    min = min.min(depth);
                }
                // left child is a leaf node, right is internal
                (None, Some(right)) => {
                    min = min.min(depth);
                    todo.push((self.codes[right], depth + 1));
                }
                // right child is a leaf, left is internal
                (Some(left), None) => {
                    min = min.min(depth);
                    todo.push((self.codes[left], depth + 1));
                }
                // both children are internal nodes
                (Some(left), Some(right)) => {
                    todo.push((self.codes[left], depth + 1));
                    todo.push((self.codes[right], depth + 1));
                }
            }
        }
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use crate::huffman_code::{CodeNode, HuffmanEncoding};
    use std::vec;

    #[test]
    fn test_encode() {
        let weights = vec![1, 2, 4, 5];
        let encoding = HuffmanEncoding {
            codes: vec![
                CodeNode {
                    left: None,
                    right: None,
                },
                CodeNode {
                    left: Some(0),
                    right: None,
                },
                CodeNode {
                    left: None,
                    right: Some(1),
                },
            ],
        };
        assert_eq!(encoding, HuffmanEncoding::encode(weights))
    }

    #[test]
    fn test_minmax() {
        let encoding = HuffmanEncoding {
            codes: vec![
                CodeNode {
                    left: None,
                    right: None,
                },
                CodeNode {
                    left: Some(0),
                    right: None,
                },
                CodeNode {
                    left: None,
                    right: Some(1),
                },
            ],
        };
        assert_eq!(encoding.min_max_length(), (1, 3))
    }
}
