use crate::huffman_code::HuffmanEncoding;

mod huffman_code;

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

    println!("Minimum codeword length is: {}", min_len)
}
