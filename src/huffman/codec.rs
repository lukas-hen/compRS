use crate::huffman::bits;

use super::bits::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::io::{prelude::*};
use std::io::Read;
use std::u32;
use std::{error::Error, result::Result};


pub fn encode<R: Read>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>> {
    
    let mut bytes = Vec::new();
    let n_read= reader.read_to_end(&mut bytes)?;

    let tree = HuffmanTree::from(bytes.as_slice());
    let lens = tree.get_bitlengths();
    let codes = generate_codes(&lens);

    let mut encoded = Vec::new();

    encoded.write(&n_read.to_be_bytes())?; // Add syms to decode. Platform specific, should hardcode size.
    encoded.write(&lens)?;  // Add canonical alphabet lengths.

    { // Need to scope the bitwriter so that we can move the encoded vec out.
        let mut bw = BitWriter::new(&mut encoded);

        for b in bytes.into_iter() {
            //println!("Char: {}, Code: {}, Len: {}", *b as char, codes[*b as usize].code, codes[*b as usize].len);
            bw.write_bits(codes[b as usize].code, codes[b as usize].len)?;
        }
    }
    
    Ok(encoded)
}

pub fn decode<R: Read>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>> {
    
    // Get n symbols.
    let mut n_syms_buf = [0u8; 8]; // Platform sensitive.
    reader.read(&mut n_syms_buf)?;
    let n_syms = usize::from_be_bytes(n_syms_buf);

    let mut lens: [u8; 256] = [0; 256];
    reader.read(&mut lens)?;

    let alphabet = generate_codes(&lens);
    let codes: Vec<Code> = alphabet
        .into_iter()
        .filter(|c| c.len > 0)
        .collect();

    const BUFSIZE: usize = 4;
    let mut buf = [0u8; BUFSIZE];

    let mut n = reader.read(&mut buf)?;
    let mut window = u32::from_be_bytes(buf);
    n = reader.read(&mut buf)?;
    
    let mut current_shift: usize = 0;
    let mut current_byte: usize = 0;    
    let mut n_syms_read = 0;
    
    let mut decoded = Vec::new();

    { 

        loop {

            let code = match_code(window, &codes)?;
            n_syms_read += 1;
            
            window <<= code.len;
            current_shift += code.len as usize;
            
            decoded.write(&[code.symbol.unwrap() as u8])?;

            if current_shift >= 8 {

                if current_byte == 4 {
                    n = reader.read(&mut buf)?;
                    current_byte = 0;
                }
                
                let mut new_byte: u32 = buf[current_byte] as u32;
                // window &= (1 << 8) - 1; // clear bits
                new_byte <<= current_shift - 8; // adjust potential offset
                window |= new_byte;

                current_byte += 1;
                current_shift -= 8;
            }

            if n_syms_read == n_syms {
                break;
            }
        }
    }

    Ok(decoded)
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    freq: usize,
    symbol: Option<u8>,
    left: Option<usize>,
    right: Option<usize>,
    depth: Option<u8>,
}

impl Node {
    fn new(freq: usize, symbol: Option<u8>, left: Option<usize>, right: Option<usize>) -> Self {
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
    nodes: Vec<Node>,
}

impl HuffmanTree {
    fn from(bytes: &[u8]) -> Self {
        let freqs = build_freq_table(bytes);
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();

        for (symbol, freq) in freqs {
            heap.push(Node::new(freq, Some(symbol), None, None))
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
        if last.is_some() {
            nodes.push(last.unwrap())
        }

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

#[derive(Clone)]
struct Code {
    code: u32,
    len: u8,
    symbol: Option<char>,
}

impl Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({:?}, {}, {:#b})\n", self.symbol, self.len, self.code).as_str())
    }
}

fn generate_codes(lens: &[u8; 256]) -> Vec<Code> {
    // Count number of symbols that have a certain length (in bits).
    let mut bl_count: [u32; 256] = [0; 256];
    for len in lens {
        bl_count[*len as usize] += 1;
    }
    bl_count[0] = 0;

    // Calculate initial values for each unique length (in bits).
    let mut next_code: [u32; 256] = [0; 256];
    let mut code: u32 = 0;

    for bitlen in 1..255 {
        // 255 absolute worst case len.
        code = (code + bl_count[bitlen - 1] as u32) << 1;
        next_code[bitlen] = code;
    }

    // Generate the actual codes.
    let mut codes: Vec<Code> = vec![Code { code: 0, len: 0, symbol: None }; 256];
    codes.reserve_exact(256);

    for n in 0..255 {
        let bl = lens[n];
        if bl != 0 {
            codes[n] = Code {
                code: next_code[bl as usize],
                len: bl,
                symbol: Some(n as u8 as char),
            };
            next_code[bl as usize] += 1;
        }
    }

    codes
}

fn match_code(window: u32, codes: &[Code]) -> Result<Code, Box<dyn Error>> {

    for code in codes.iter() {
    
        let c = code.code << (32 - code.len);
        let mask = gen_bitmask_lalign(code.len);
        if (window & mask) == c {
            return Ok(code.clone());
        }
    }

    Err("Could not match code.".into())
}

fn build_freq_table(symbols: &[u8]) -> HashMap<u8, usize> {
    let mut freqs: HashMap<u8, usize> = HashMap::new();

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

fn gen_bitmask_lalign(sz: u8) -> u32 {
    if sz == 0 {
        return 0;
    } // Protect from overflow panic.
    let mask = u32::MAX >> 32 - sz; // right aligned
    mask.reverse_bits()
}
