use std::fs;

mod huffman;

fn main() {
    let file_path: &str = "./resources/poem.txt";

    let contents = fs::read(file_path).expect("Should have been able to read the file");
    huffman::to_dotfile(contents.as_slice(), "./resources/test.dot").expect("Failed to write dotfile");
}
