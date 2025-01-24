use std::collections::{BinaryHeap, HashMap};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::cmp::Ordering;

// Node for Huffman Tree
#[derive(Debug, Eq)]
pub struct Node {
    frequency: usize,
    character: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Implement PartialEq and Ord for BinaryHeap
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse to make BinaryHeap a min-heap
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Build frequency map
pub fn build_frequency_map(input: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();
    for c in input.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }
    freq_map
}

// Build Huffman Tree
pub fn build_huffman_tree(freq_map: &HashMap<char, usize>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    // Push all characters as leaf nodes
    for (&character, &frequency) in freq_map {
        heap.push(Node {
            frequency,
            character: Some(character),
            left: None,
            right: None,
        });
    }

    // Build the tree
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let parent = Node {
            frequency: left.frequency + right.frequency,
            character: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        heap.push(parent);
    }

    heap.pop().map(Box::new)
}

// Generate Huffman Codes
pub fn generate_huffman_codes(
    node: &Option<Box<Node>>,
    prefix: String,
    codes: &mut HashMap<char, String>,
) {
    if let Some(node) = node {
        if let Some(character) = node.character {
            codes.insert(character, prefix);
        } else {
            generate_huffman_codes(&node.left, format!("{}0", prefix), codes);
            generate_huffman_codes(&node.right, format!("{}1", prefix), codes);
        }
    }
}

// Compress the file
pub fn compress(input: &str, codes: &HashMap<char, String>) -> String {
    input.chars().map(|c| codes[&c].clone()).collect()
}

// Save compressed file and codes
pub fn save_compressed_file(filename: &str, compressed_data: &str, codes: &HashMap<char, String>) -> io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "{}", compressed_data)?;
    writeln!(file, "{:?}", codes)?;
    Ok(())
}

// Read a file into a string
pub fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}