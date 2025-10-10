use std::collections::{BinaryHeap, HashMap};
use std::{error::Error, result::Result};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

pub fn encode(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    
    let tree = HuffmanTree::from(bytes);
    let lens = tree.get_bitlengths();
    let codes = generate_codes(&lens);

    let mut f = File::create("compressed")?;
    let _ = f.write(&lens)?;

    let mut bw = BitWriter::new(f);

    for b in bytes {
        bw.write_bits(codes[*b as usize], lens[*b as usize])?;
    }

    Ok(())
}

pub fn decode(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut f = File::open("./compressed").expect("Could not open file.");
    let mut lens: [u8; 256] = [0; 256];
    let _n_read = f.read(&mut lens)?;

    let alphabet = generate_codes(&lens);
    let codes: Vec<u32> = alphabet.into_iter().filter(|l| l.gt(&0u32)).collect();

    let mut buf = [0u8; 128];
    
    while let Ok(n_read) = f.read(&mut buf) {
        if n_read == 0 {
            break;
        }
    
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    freq: u32,
    symbol: Option<u8>,
    left: Option<usize>,
    right: Option<usize>,
    depth: Option<u8>,
}

impl Node {

    fn new(
        freq: u32,
        symbol: Option<u8>,
        left: Option<usize>,
        right: Option<usize>,
    ) -> Self {
        Self {
            freq,
            symbol,
            left,
            right,
            depth: None,
        }
    }

    fn from(l_child: &Node, l_idx: usize, r_child: &Node, r_idx: usize) -> Self {
        Self {
            freq: l_child.freq + r_child.freq,
            symbol: None,
            left: Some(l_idx),
            right: Some(r_idx),
            depth: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Ordering to make BinaryHeap a min-heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .freq
            .cmp(&self.freq)
            .then_with(|| self.symbol.cmp(&other.symbol))
    }
}

#[derive(Debug)]
struct HuffmanTree {
    nodes: Vec<Node>
}

impl HuffmanTree {

    fn from(bytes: &[u8]) -> Self {

        let freqs = build_freq_table(bytes);
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();

        for (symbol, freq) in freqs {
            heap.push(
                Node::new(freq, Some(symbol), None, None)
            )
        }

        let mut nodes: Vec<Node> = Vec::new();
        let mut idx: usize = 0;

        while heap.len() > 1 {

            let l = heap.pop().unwrap();
            let r = heap.pop().unwrap();

            let l_idx = idx;
            let r_idx = idx + 1;

            let p = Node::from(&l, l_idx, &r, r_idx);

            nodes.push(l);
            nodes.push(r);
            heap.push(p);

            idx += 2;

        }

        let last = heap.pop();
        if last.is_some() { nodes.push(last.unwrap())}

        let mut tree = Self { nodes };
        
        tree.set_depths();

        tree

    }

    pub fn set_depths(&mut self) -> () {

        // Iterate with DFS to calculcate depths in one pass
        // Doesn't work in level order as tree is not necessarily complete. 
        // (It's a _full_ binary tree.)

        let mut node_idx_stack: Vec<usize> = Vec::new();
        let mut depth_stack: Vec<u8> = Vec::new();

        let root_idx = self.nodes.len() - 1;
        node_idx_stack.push(root_idx);

        // Assume AT LEAST 3 nodes.
        depth_stack.push(1);
        depth_stack.push(1);
        depth_stack.push(0);

        while let Some(node_idx) = node_idx_stack.pop() {

            let depth = depth_stack.pop().unwrap();

            if self.nodes[node_idx].is_leaf() {
                // If leaf, skip. 
                // every node has exactly 0 or 2 children, thus fine to only check if leaf.
                self.nodes[node_idx].depth = Some(depth);
                continue;
            }

            self.nodes[node_idx].depth = Some(depth);

            node_idx_stack.push(self.nodes[node_idx].right.unwrap());
            node_idx_stack.push(self.nodes[node_idx].left.unwrap());
            depth_stack.push(depth + 1);
            depth_stack.push(depth + 1);
        }

    }

    fn get_bitlengths(&self) -> [u8; 256] {
        let mut lens: [u8; 256] = [0; 256];

        for node in self.nodes.iter() {
            
            match node.symbol {
                Some(s) => lens[s as usize] = node.depth.unwrap(),
                None => continue,
            }

        }
        
        lens
    }

}



fn generate_codes(lens: &[u8; 256]) -> [u32; 256] {
        
        // Count number of symbols that have a certain length (in bits).
        let mut bl_count: [u32; 256] = [0; 256];
        for len in lens {
            bl_count[*len as usize] += 1;
        }
        bl_count[0] = 0;

        // Calculate initial values for each unique length (in bits).
        let mut next_code: [u32; 256] = [0; 256];
        let mut code: u32 = 0;

        for bitlen in 1..255 { // 255 absolute worst case len.
            code = (code + bl_count[bitlen-1] as u32) << 1;
            next_code[bitlen] = code;
        }

        // Generate the actual codes.
        let mut codes: [u32; 256] = [0; 256];
        for n in 0..255 {
            let bl = lens[n];
            if bl != 0 {
                codes[n] = next_code[bl as usize];
                next_code[bl as usize] += 1;
            }
        }
        
        codes
            
    }


fn build_freq_table(symbols: &[u8]) -> HashMap<u8, u32> {
    let mut freqs: HashMap<u8, u32> = HashMap::new();

    for symbol in symbols {
        match freqs.get(symbol) {
            Some(count) => {
                freqs.insert(symbol.to_owned(), count + 1);
            }
            None => {
                freqs.insert(symbol.to_owned(), 1);
            }
        }
    }

    freqs
}



pub struct BitWriter {
    file: File,
    buffer: u8,
    buffer_len: u8, // Number of bits currently in the buffer
}

impl BitWriter {
    /// Create a new `BitWriter` that writes to the given file.
    pub fn new(file: File) -> Self {
        Self {
            file,
            buffer: 0,
            buffer_len: 0,
        }
    }

    /// Write the least significant `num_bits` of `value` to the output.
    pub fn write_bits(&mut self, value: u32, num_bits: u8) -> Result<(), Box<dyn Error>> {

        let mut bits_to_write = num_bits;
        let mut value = value & ((1 << num_bits) - 1); // Mask only the necessary bits of value

        while bits_to_write > 0 {
            let space_left = 8 - self.buffer_len;
            let bits_to_copy = bits_to_write.min(space_left);

            // Copy the top `bits_to_copy` bits into the buffer
            self.buffer |= ((value >> (bits_to_write - bits_to_copy)) as u8) << (space_left - bits_to_copy);

            bits_to_write -= bits_to_copy;
            self.buffer_len += bits_to_copy;

            // If the buffer is full, write it to the file
            if self.buffer_len == 8 {
                self.flush_buffer()?;
            }

            // Clear the written bits from `value`
            value &= (1 << bits_to_write) - 1;
        }

        Ok(())
    }

    /// Flush the remaining bits in the buffer to the file, padding with zeros if necessary.
    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        if self.buffer_len > 0 {
            self.flush_buffer()?;
        }
        self.file.flush()?;
        Ok(())
    }

    /// Internal method to flush the buffer (only if it's full or on `flush` call).
    fn flush_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.write_all(&[self.buffer])?;
        self.buffer = 0;
        self.buffer_len = 0;
        Ok(())
    }

}

impl Drop for BitWriter {
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            eprintln!("Failed to flush BitWriter: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn bit_writer_test() {
        let file_path = "test_output.bin";
        let file = File::create(file_path).unwrap();
        let mut writer = BitWriter::new(file);

        // Write some values using varying bit lengths
        writer.write_bits(0b101, 3).unwrap();
        writer.write_bits(0b11111111, 8).unwrap();
        writer.write_bits(0b1, 1).unwrap();
        writer.write_bits(0b10, 2).unwrap();
        writer.flush().unwrap();

        let written_data = fs::read(file_path).unwrap();
        assert_eq!(written_data, vec![0b10111111, 0b11111000]);

        // Clean up
        fs::remove_file(file_path).unwrap();
    }
}