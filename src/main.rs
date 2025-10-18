use std::fs;

mod huffman;

fn main() {
    let filename: &str = "./resources/book1.txt";
    let contents = fs::read(filename).expect("Should have been able to read the file");

    // huffman_dotfile::write(contents.as_slice(), "./resources/test.dot")
    //     .expect("Failed to write dotfile");

    let _ = huffman::codec::encode(contents.as_slice());

    let compr = fs::read("./compressed").expect("Should have been able to read the file");
    let _ = huffman::codec::decode(compr.as_slice());
}
