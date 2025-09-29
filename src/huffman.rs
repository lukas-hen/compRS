use std::collections::{BinaryHeap, HashMap};
use std::{error::Error, result::Result};
use std::cmp::Ordering;
use std::fmt::Debug;

const ALPHABET_SZ_ASCII: usize = 256;
const F_BINTREE_SZ_ASCII: usize = 2*ALPHABET_SZ_ASCII-1;

pub fn encode(bytes: &[u8], filename: &str) -> Result<(), Box<dyn Error>> {
    let tree = HuffmanTree::from(bytes);
    //println!("{:?}", tree);
    let _ = tree.get_lengths();

    Ok(())
}

pub fn decode(bytes: &[char], filename: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    freq: u32,
    symbol: Option<u8>,
    left: Option<usize>,
    right: Option<usize>,
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
        }
    }

    fn from(l_child: &Node, l_idx: usize, r_child: &Node, r_idx: usize) -> Self {
        Self {
            freq: l_child.freq + r_child.freq,
            symbol: None,
            left: Some(l_idx),
            right: Some(r_idx),
        }
    }

    #[inline]
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

        Self { nodes }
    }

    fn get_lengths(&self) -> Vec<u8> {
        
        // depth
        let mut d = 0;
        // current node in given depth
        let mut n = 0;
        
        let lens = Vec::new();
        lens.reserve(128);

        for node in self.nodes.iter().rev() {
            
            if n >= tree_depth_size(d) {
                d += 1;
                n = 0;
            }
            
            lens[d] += 
            println!("Depth: {d}, N: {n}, SYMFREQ {}", node.freq);
            
            if node.is_leaf() {
                n += 2;
            } else {
                n += 1;
            }

        }

        vec![(1u8, 1u8)]
    }

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

#[inline]
fn tree_depth_size(d: usize) -> usize {
    // 0 indexed depth max width for a perfect bintree.
    1 << d
}

#[test]
fn tree_depth_calc() {
    assert_eq!(1, tree_depth_size(0));
    assert_eq!(2, tree_depth_size(1));
    assert_eq!(4, tree_depth_size(2));
    assert_eq!(8, tree_depth_size(3));
    assert_eq!(16, tree_depth_size(4));
}