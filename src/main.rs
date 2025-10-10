use std::{fs};

mod huffman_dotfile;
mod huffman;

fn main() {
    let filename: &str = "./resources/tiny_alphabet.txt";
    let contents = fs::read(filename).expect("Should have been able to read the file");

    // huffman_dotfile::write(contents.as_slice(), "./resources/test.dot")
    //     .expect("Failed to write dotfile");
    
    let _ = huffman::encode(contents.as_slice());
    
    let compr = fs::read("./compressed").expect("Should have been able to read the file");
    let _ = huffman::decode(compr.as_slice());
}
