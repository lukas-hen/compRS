# Huffman Encoding/Decoding CLI

`huff` is a CLI tool for huffman encoding & decoding.

The huffman implementation efficiently compresses and decompresses files or in-memory buffers using canonical Huffman encoding.

## Implementation Specifics

1. **No chunked reading off file**:  
   The implementation reads the entire input into memory before processing. There is currently no support for chunking large files, so this solution is best suited for small to medium-sized data.
    _Adjusting the implementation to chunk the file-reading should not be very difficult, but for now this is not a feature._

2. **Byte-Based Encoding**:  
   Only individual bytes are treated as symbols. Multi-byte encoding formats like UTF-8 are not yet supported.  

3. **Canonical Huffman Trees**:  
   The Huffman tree is serialized in canonical form, as described in the Deflate RFC. Instead of storing the tree structure, only the bit lengths of each symbol are stored. This approach:  
   - Reduces the storage footprint of the tree.  
   - Enables deterministic reconstruction of the tree during decoding.  
   - Improves decoding efficiency by ordering symbols with shorter bit lengths (higher frequencies) first.  

4. **File Format Layout**:  
   Compressed files are laid out as follows:  
   - **Header**: Records the number of symbols encoded (stored as a `usize`).  
   - **Canonical Tree Data**: Contains the bit lengths for each symbol in the alphabet.  
   - **Encoded Data**: Represents the compressed payload, stored as packed bits.  

5. **Efficient Decoding**:  
   Instead of traversing the Huffman tree bit-by-bit, the decoder peeks at a sliding window of bits and matches patterns in order of the canonical tree. This method pretty efficient (_Compared to traversing the tree for each individual bit_) and tries the most frequent symbols first, reducing computational overhead.

## Installation

### Installing the CLI Tool

1. Clone the GitHub repository:  
    ```bash
    git clone https://github.com/lukas-hen/huffman_rs.git
    cd huffman_rs
    ```

2. Use `cargo` to install the CLI tool:  
    ```bash
    cargo install --path .
    ```

After installation, the `huff` CLI will be available for use on your system.

## CLI Usage

```plaintext
Usage: huff <COMMAND>

Commands:
  encode  Encode a file
  decode  Decode a file
  dot     Generate a .dot file for visualizing the huffman tree of a given input
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

#### Compressing a File

To compress a file named `input.txt` into a compressed file `output.compressed`:

```bash
huff encode -i input.txt -o output.compressed
```

#### Decompressing a File

To decompress the file `output.compressed` into `decompressed.txt`:

```bash
huff decode -i output.compressed -o decompressed.txt
```

## Contributing

Contributions are welcome! Feel free to open issues for suggestions, bug reports, or feature requests.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute this project as you see fit.

## Notes

This project is not yet in a production-ready state and built as a fun exercise in rust & compression algorithms. ❤️
