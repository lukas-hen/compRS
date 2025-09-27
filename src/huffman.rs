use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::fmt::Display;
use std::format;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::{error::Error, result::Result};

// ------------- API -------------

pub fn encode(input: &[u8], filename: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn decode(input: &[u8], filename: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn write_dotfile(bytes: &[u8], filename: &str) -> Result<(), Box<dyn Error>> {
    let tree = HuffmanTree::new(bytes);
    //tree.write_dotfile(filename)?;

    for n in tree.iter_preorder() {
        println!("{:?}", n);
    }

    Ok(())
}

// ------------- PRIV -------------

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    id: usize,
    freq: usize,
    symbol: Option<u8>,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(
        id: usize,
        freq: usize,
        symbol: Option<u8>,
        left: Option<usize>,
        right: Option<usize>,
    ) -> Self {
        Self {
            id,
            freq,
            symbol,
            left,
            right,
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

struct HuffmanTree {
    nodes: Vec<Node>,
}

struct PreOrderIterator<'a> {
    nodes: &'a Vec<Node>,
    stack: Vec<&'a Node>,
    pos: usize,
}

impl Iterator for PreOrderIterator<'_> {
    
    type Item=Node;

    fn next(&mut self) -> Option<Self::Item> {
        
        let node = self.stack.pop()?;

        if node.right.is_some() {
            self.stack.push(&self.nodes[node.right.unwrap()]);
        }
        
        if node.left.is_some() {
            self.stack.push(&self.nodes[node.left.unwrap()]);
        }

        self.pos += 1;

        Some(node.clone())
    }
}

// impl<'a> IntoIterator for &'a HuffmanTree {

//     type Item=Node;
//     type IntoIter = PreOrderIterator<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         PreOrderIterator{
//             data: &self.nodes, 
//             stack: 
//             pos: 0
//         }
//     }
// }

impl HuffmanTree {
    
    fn new(bytes: &[u8]) -> Self {
        let freqs = build_freq_table(bytes);
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();

        // Required unique id per node for writing dotfiles.
        let mut id: usize = 0;

        for (symbol, freq) in freqs {
            heap.push(Node::new(
                id,
                freq.to_owned(),
                Some(symbol.to_owned()),
                None,
                None,
            ));
            id += 1;
        }

        let mut nodes: Vec<Node> = Vec::new();
        let mut idx: usize = 0;

        while !heap.is_empty() {
            let left = heap.pop().unwrap();
            let right_opt = heap.pop();

            if right_opt.is_none() {
                // Last node
                nodes.push(left);
                break;
            }

            let right = right_opt.unwrap();

            let combined_freq = left.freq + right.freq;

            nodes.push(left);
            nodes.push(right);

            let parent_node = Node::new(id, combined_freq, None, Some(idx), Some(idx + 1));
            idx += 2;
            id += 1;

            heap.push(parent_node);
        }

        Self { nodes }
    }

    fn write_dotfile(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let f = File::create(filename)?;
        let mut f = BufWriter::new(f);

        write_dotfile_header(&mut f)?;

        // Write nodes
        for node in &self.nodes {
            if node.symbol.is_some() {
                write_dotfile_label(&mut f, node.id, node.symbol.unwrap() as char)?
            } else {
                write_dotfile_label(&mut f, node.id, node.freq)?
            }
        }

        // Write edges
        for node in &self.nodes {

            if node.left.is_none() && node.right.is_none() {
                continue;
            }

            f.write(format!("\tl{} -> {{ ", node.id).as_bytes())?;

            if let Some(idx) = node.left {
                let l = self.nodes[idx].id;
                f.write(format!("l{l}").as_bytes())?;
            }

            if let Some(idx) = node.right {
                let r = self.nodes[node.right.unwrap()].id;
                f.write(format!(" l{r}").as_bytes())?;
            }

            f.write(" };\n".as_bytes())?;

        }

        write_dotfile_closure(&mut f)?;

        Ok(())
    }

    fn iter_preorder(&self) -> PreOrderIterator {
        let mut stack = Vec::new();
        stack.push(self.nodes.last().unwrap());
        PreOrderIterator{ 
            nodes: &self.nodes, 
            stack: stack,
            pos: 0
        }
    }
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

fn write_dotfile_header<T: Write>(w: &mut T) -> Result<(), Box<dyn Error>> {
    w.write("digraph BST {\n\tnode [fontname=\"Arial\"]\n".as_bytes())?;
    Ok(())
}

fn write_dotfile_label<T, I, L>(w: &mut T, node_id: I, symbol: L) -> Result<(), Box<dyn Error>>
where
    T: Write,
    I: Display,
    L: Display,
{
    let s = format!("\tl{} [ label = \"{}\" ];\n", node_id, symbol,);

    w.write(s.as_bytes())?;

    Ok(())
}

fn write_dotfile_closure<T: Write>(w: &mut T) -> Result<(), Box<dyn Error>> {
    w.write("}".as_bytes())?;
    Ok(())
}
