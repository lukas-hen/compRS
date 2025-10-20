use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

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
pub struct HuffmanTree {
    nodes: Vec<Node>,
}

impl HuffmanTree {
    pub fn from(bytes: &[u8]) -> Self {
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

    fn set_depths(&mut self) -> () {
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

    pub fn get_bitlengths(&self) -> [u8; 256] {
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
