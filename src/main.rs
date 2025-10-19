use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io::{BufWriter, Write};

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
    /// If not set, the default behaviour is to compress.
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
        Commands::Huffman(HuffmanArgs {
            input,
            output,
            decompress: true,
        }) => {
            huffman_decode_file(input, output);
        }

        Commands::Huffman(HuffmanArgs {
            input,
            output,
            decompress: false,
        }) => {
            huffman_encode_file(input, output);
        }

        _ => {}
    }
}

fn huffman_decode_file(input_fname: &String, output_fname: &String) {
    let mut f_in = File::open(input_fname).unwrap();
    let decoded = huffman::codec::decode(&mut f_in).unwrap();
    let mut f_out = File::create(output_fname).unwrap();
    let mut bw = BufWriter::new(f_out);
    bw.write_all(decoded.as_slice()).unwrap();
}

fn huffman_encode_file(input_fname: &String, output_fname: &String) {
    let mut f_in = File::open(input_fname).unwrap();
    let encoded = huffman::codec::encode(&mut f_in).unwrap();
    let mut f_out = File::create(output_fname).unwrap();
    let mut bw = BufWriter::new(f_out);
    let _ = bw.write_all(encoded.as_slice()).unwrap();
}
