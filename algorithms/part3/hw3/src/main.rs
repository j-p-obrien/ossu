mod huffman_code;
mod mwis;

use crate::huffman_code::HuffmanEncoding;
use crate::mwis::MWIS;

const VERTICES_TO_CHECK: [usize; 8] = [0, 1, 2, 3, 16, 116, 516, 996];

fn main() {
    let weights: Vec<usize> = std::fs::read_to_string("huffman.txt")
        .unwrap()
        .lines()
        .skip(1)
        .map(|w| w.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let codes = HuffmanEncoding::encode(weights);

    let (min_len, max_len) = codes.min_max_length();

    println!("Maximum codeword length is: {}", max_len);

    println!("Minimum codeword length is: {}", min_len);

    let weights: MWIS = MWIS {
        weights: std::fs::read_to_string("mwis.txt")
            .unwrap()
            .lines()
            .skip(1)
            .map(|w| w.parse())
            .collect::<Result<_, _>>()
            .unwrap(),
    };

    let vertices = weights.mwis();
    // result_string[i] = 1 if vertex i included, 0 otherwise
    let mut result_string = String::new();

    for vertex in VERTICES_TO_CHECK {
        result_string.push_str(if vertices[vertex] { &"1" } else { &"0" })
    }
    println!("String to enter is: {result_string}");
}
