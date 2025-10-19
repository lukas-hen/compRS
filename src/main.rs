use std::fs;
mod huffman;

fn main() {

    let filename: &str = "./resources/poem.txt";
    let contents = fs::read(filename).expect("Should have been able to read the file");

    // huffman_dotfile::write(contents.as_slice(), "./resources/test.dot")
    //     .expect("Failed to write dotfile");

    let encoded = huffman::codec::encode(&mut contents.as_slice()).expect("Encoding failed");
    let decoded = huffman::codec::decode(&mut encoded.as_slice()).expect("Decoding failed");

    assert_eq!(contents.as_slice(), decoded.as_slice());
    
}
