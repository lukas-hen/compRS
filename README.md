# compRS

`compRS` is a Rust library & CLI tool for compression & decompression.
Currently the only algorithm implemented is Huffman encoding and decoding. 

The huffman implementation efficiently compresses and decompresses files or in-memory buffers using canonical Huffman encoding, providing both a CLI and a simple API.

In the future more compression methods might be added, thus the repo is not a huffman specific repo. The CLI only has a subcommand `huffman` for huffman-encoding for now.

## Features

- Support for encoding/decoding **files** using the CLI.
- Support for encoding/decoding **in-memory buffers** via the huffman module.

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
    git clone https://github.com/<your-username>/compRS.git
    cd compRS
    ```

2. Use `cargo` to install the CLI tool:  
    ```bash
    cargo install --path .
    ```

After installation, the `comprs` CLI will be available for use on your system.

## CLI Usage

As i might grow this to other compression algorithms in the future, all huffman-related args are under the subcommand `huffman`.

```plaintext
Encode file using Huffman Encoding
Usage: comprs huffman [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -d, --decompress       If not set, the default behaviour is to compress
  -i, --input <INPUT>    Input filename
  -o, --output <OUTPUT>  Compressed filename
  -h, --help             Print help
```

### Examples

#### Compressing a File

To compress a file named `input.txt` into a compressed file `output.compressed`:

```bash
comprs huffman --input input.txt --output output.compressed
```

#### Decompressing a File

To decompress the file `output.compressed` into `decompressed.txt`:

```bash
comprs huffman --decompress --input output.compressed --output decompressed.txt
```

## Library API

You can use `compRS` programmatically in your own Rust projects by leveraging the `encode` and `decode` functions provided in the `huffman` module.

### API Overview

#### `encode`

```rust
pub fn encode<R: Read>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>>
```

- **Description**: Encodes input using canonical Huffman encoding.
- **Parameters**:  
  - `reader`: A mutable reference to any type implementing the `Read` trait (e.g., a file, a buffer).
- **Returns**: A `Result` containing the encoded data as a `Vec<u8>` or an error.

#### `decode`

```rust
pub fn decode<R: Read>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>>
```

- **Description**: Decodes a buffer or file previously encoded with Huffman encoding.
- **Parameters**:  
  - `reader`: A mutable reference to any type implementing the `Read` trait (e.g., a file, a buffer).
- **Returns**: A `Result` containing the decoded data as a `Vec<u8>` or an error.

## Contributing

Contributions are welcome! Feel free to open issues for suggestions, bug reports, or feature requests.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute this project as you see fit.

## Notes

This project is not yet in a production-ready state and built as a fun exercise in rust & compression algorithms. ❤️
