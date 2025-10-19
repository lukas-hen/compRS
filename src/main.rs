use clap::{Args, Command, Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

mod huffman;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Compression method
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encode file using Huffman Encoding
    Huffman(HuffmanArgs),
}

#[derive(Args, Debug)]
struct HuffmanArgs {
    /// If not set default behaviour is to compress.
    #[arg(short, long)]
    decompress: bool,

    /// Input filename
    #[arg(short, long)]
    input: String,

    /// Compressed filename
    #[arg(short, long)]
    output: String,
}


fn main() {

    let cli = Cli::parse();


    match &cli.command {
        Commands::Huffman(HuffmanArgs{input, output, decompress: true}) => {println!("DECOMPRESS")}
        Commands::Huffman(HuffmanArgs{input, output, decompress: false}) => {println!("COMPRESS")}
        _ => {}
    }

    // let filename: &str = "./resources/poem.txt";
    // let contents = fs::read(filename).expect("Should have been able to read the file");

    // // huffman_dotfile::write(contents.as_slice(), "./resources/test.dot")
    // //     .expect("Failed to write dotfile");

    // let encoded = huffman::codec::encode(&mut contents.as_slice()).expect("Encoding failed");
    // let decoded = huffman::codec::decode(&mut encoded.as_slice()).expect("Decoding failed");

    // assert_eq!(contents.as_slice(), decoded.as_slice());
}
