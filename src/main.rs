use std::fs;

mod huffman;

fn main() {
    let file_path: &str = "./resources/poem.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let freqs = huffman::to_dot(&contents.chars().collect::<Vec<char>>(), "./resources/test.dot").expect("Failed to write dotfile");

    println!("{:?}", freqs);
}
