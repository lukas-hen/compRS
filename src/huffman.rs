use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::fs::File;
use std::format;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::{error::Error, result::Result};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node<T>
where
    T: Ord + Clone + Display + Debug,
{
    id: usize,
    freq: usize,
    symbol: Option<T>,
    left: Option<usize>,
    right: Option<usize>,
}

impl<T> Node<T>
where
    T: Ord + Clone + Display + Debug,
{
    fn new(id: usize, freq: usize, symbol: Option<T>, left: Option<usize>, right: Option<usize>) -> Self {
        Self {
            id,
            freq,
            symbol,
            left,
            right,
        }
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Ord + Clone + Display + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Ordering to make BinaryHeap a min-heap
impl<T> Ord for Node<T>
where
    T: Ord + Clone + Display + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .freq
            .cmp(&self.freq)
            .then_with(|| self.symbol.cmp(&other.symbol))
    }
}

struct HuffmanTree<T>
where
    T: Ord + Clone + Display + Debug,
{
    nodes: Vec<Node<T>>,
}

impl<T> HuffmanTree<T>
where
    T: Ord + Clone + Display + Debug,
{

    fn from_freq_table(freq_table: &HashMap<T, usize>) -> Option<Self> {
        let mut heap: BinaryHeap<Node<T>> = BinaryHeap::new();
        let mut id: usize = 0;

        for (symbol, freq) in freq_table {
            heap.push(Node::new(id, freq.to_owned(), Some(symbol.to_owned()), None, None));
            id += 1;
        }

        let mut nodes: Vec<Node<T>> = Vec::new();
        let mut idx: usize = 0;

        while !heap.is_empty() {
        
            let left = heap.pop()?;
            let right_opt = heap.pop();
            
            if right_opt.is_none() { 
                // Last node
                nodes.push(left);
                break;
            }

            let right = right_opt?;

            let combined_freq = left.freq + right.freq;

            nodes.push(left);
            nodes.push(right);

            let parent_node = Node::new(id, combined_freq, None, Some(idx), Some(idx + 1));
            idx += 2;
            id += 1;

            heap.push(parent_node);
        }

        Some(Self { nodes })
    }

    fn to_dotfile(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let f = File::create(filename).expect("Should be able to create file");
        let mut f = BufWriter::new(f);

        f.write("digraph BST {\n\tnode [fontname=\"Arial\"]\n".as_bytes())?;
        
        for node in &self.nodes {
            if node.symbol.is_some() {
                let s = format!("\tl{} [ label = \"{}\" ];\n", node.id, node.symbol.as_ref().unwrap());
                f.write(s.as_bytes())?;
            } else {
                let s = format!("\tl{} [ label = \"[{}]\" ];\n", node.id, node.freq);
                f.write(s.as_bytes())?;
            }
            
        }

        
        for node in &self.nodes {

            if node.left.is_none() && node.right.is_none() {
                continue;
            }

            f.write(format!("\tl{} -> {{ ", node.id).as_bytes())?;

            if node.left.is_some() {
                let l = self.nodes[node.left.unwrap()].id;
                f.write(format!("l{}", l).as_bytes())?;
            }

            if node.right.is_some() {
                f.write(" ".as_bytes())?;
                let r = self.nodes[node.right.unwrap()].id;
                f.write(format!("l{}", r).as_bytes())?;
            }
            
            f.write(" };\n".as_bytes())?;
        }

        f.write("}".as_bytes())?;

        Ok(())
    }
}

fn build_freq_table<T>(symbols: &[T]) -> HashMap<T, usize>
where
    T: Eq + Hash + Clone + Display,
{
    let mut freqs: HashMap<T, usize> = HashMap::new();

    symbols.iter().for_each(|b| match freqs.get(b) {
        Some(count) => {
            freqs.insert(b.to_owned(), count + 1);
        }
        None => {
            freqs.insert(b.to_owned(), 1);
        }
    });

    freqs
}

// PUB

pub fn encode<T>(symbols: &[T], filename: &str) -> Vec<T>
where
    T: Eq + Hash + Ord + Clone + Display + Debug
{
    let freqs = build_freq_table(symbols);
    let tree = HuffmanTree::from_freq_table(&freqs);
    
    return Vec::new();
}

pub fn decode<T>(symbols: &[T]) -> Vec<T> 
where
    T: Eq + Hash + Ord + Clone + Display + Debug
{
    build_freq_table(symbols);
    return Vec::new();
}

pub fn to_dotfile<T>(bytes: &[T], filename: &str) -> Result<(), Box<dyn Error>> 
where
    T: Eq + Hash + Ord + Clone + Display + Debug
{
    let freqs = build_freq_table(bytes);
    let tree = HuffmanTree::from_freq_table(&freqs).unwrap();
    tree.to_dotfile(filename)?;
    Ok(())
}


