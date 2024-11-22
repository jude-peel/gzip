use std::{cmp::Ordering, collections::BinaryHeap, fmt::Display};

use crate::bitstream::BitIndex;

pub const FIXED_CODE_LENGTHS: [u8; 288] = [
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8,
];

/// A struct for representing codes of differing bit lengths, codes are stored
/// little endian, meant to be read from most significant bit to least
/// significant bit.
///
/// # Fields
///
/// * 'buffer' - A u32 acting as a bit buffer.
/// * 'length' - A u8 specifying how many of the bits in the buffer are actually
///         a part of the code. A length of 2 would mean that the 2 least
///         significant bits hold the code.
///
/// # Methods
///
/// * 'new' - Generates a new empty Code.
/// * 'from' - Accepts a buffer and a length and creates a Code struct from
///         those given values.
/// * 'push' - Accepts a buffer and a length and pushes length bits of value
///         into the bit buffer.
/// * 'push_bit' - Accepts a single u8 which is normalized to represent either
///         a 0 or 1, and pushes it to the buffer.
///
/// # Examples
///
/// '''
/// let new = Code::new();
/// let from = Code::from(0b1011, 4);
///
/// new_code.push_bit(1);
/// new_code.push(0b011, 3);
///
/// // Both codes now have a length of 4, and the u32 value:
/// // 0b0000_0000_0000_0000_0000_0000_0000_1011
/// assert_eq!(new.code, from.code);
/// '''
#[derive(Clone, Debug)]
pub struct Code {
    pub buffer: u32,
    pub length: u8,
}

impl Code {
    /// Constructs a new, empty instance of Code.
    ///
    /// # Returns
    ///
    /// A Code struct with zeroes for both fields.
    pub fn new() -> Self {
        Self {
            buffer: 0,
            length: 0,
        }
    }
    /// Constructs an instance of Code with a given code and length.
    ///
    /// # Arguments
    ///
    /// * 'code' - The u32 value containing the binary code.
    /// * 'length' - A u8 representing the number of bits of code are a part
    ///         of the binary code.
    ///
    /// # Returns
    ///
    /// A Code struct with the given values.
    pub fn from(buffer: u32, length: u8) -> Self {
        Self { buffer, length }
    }
    /// Accepts a length and a u32 as a buffer, and pushes length bits of that
    /// buffer into self.code and increments self.length by the appropriate
    /// amount.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A u32 acting as a bit buffer containing the bits to push.
    /// * 'length' - The number of bits to push.
    pub fn push(&mut self, buffer: u32, length: u8) {
        self.buffer = (self.buffer << length) | buffer;
        self.length += length;
    }
    /// Accepts either a 0 or 1 and pushes that bit to self. If a non-binary
    /// value is entered it will correct it to a 1 instead of raising an error.
    ///
    /// # Arguments
    ///
    /// * 'bit' - A u8 representing the bit to push.
    pub fn push_bit(&mut self, bit: u8) {
        let normalized_bit: u32 = match bit {
            0 => 0,
            1 => 1,
            _ => {
                eprintln!("Warning: Non-binary value passed to push, value corrected to a 1.");
                1
            }
        };

        self.buffer = (self.buffer << 1) | normalized_bit;
        self.length += 1;
    }
}

impl Default for Code {
    fn default() -> Self {
        Code::new()
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0length$b}",
            self.buffer,
            length = self.length as usize
        )
    }
}

/// Struct representing each node of a binary tree.
///
/// # Fields
///
/// * 'value' - An option containing a usize or None, None represents that
///         the node is a branch rather than a leaf, if a value is present,
///         the node should be on the edge of the tree.
/// * 'significance' - A u64 value used to sort the nodes on the tree. If
///         implementing a frequency based Huffman tree, significance can
///         be used to represent the frequency of each node. If, used to
///         generate prefix codes, significance represents the code.
/// * 'code' - An instance of the Code struct which contains a u32 bit buffer
///         containing the code, and a length representing what quantity of bits
///         in the buffer are part of the code.
/// * 'left' - An option holding a Box reference to the child node attached to
///         the left.
/// * 'right' - An option holding a Box reference to the child node attached to
///         the right.
#[derive(Debug, Clone)]
pub struct Node {
    pub value: Option<usize>,
    pub significance: u64,
    pub code: Code,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    /// Creates a new empty Node.
    ///
    /// # Returns
    ///
    /// A node with default values.
    pub fn new() -> Self {
        Self {
            value: None,
            significance: 0,
            code: Code::new(),
            left: None,
            right: None,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {})", self.value, self.code)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.significance.eq(&other.significance)
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.significance.cmp(&other.significance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

/// A binary tree containing prefix codes.
///
/// # Fields
///
/// * 'root' - The root node to which all others are connected.
/// * 'leaves' - A vector containing all the leaf nodes/nodes with values.
/// * 'current' - The most recent node to be traversed.
#[derive(Debug)]
pub struct PrefixTree {
    pub root: Node,
    pub leaves: Vec<Node>,
    pub current: Box<Node>,
}

impl PrefixTree {
    /// Generates a prefix code tree from the given bit lengths.
    ///
    /// # Arguments
    ///
    /// * 'code_lengths' An array of u8 values representing the number of
    ///         bits in the code to represent a certain symbol, the symbol
    ///         is the index of the value. So, bit_lengths[1] would equal
    ///         the length of the code for 1.
    ///
    /// # Returns
    ///
    /// A new instance of PrefixTree built from the bit lengths provided.
    pub fn from_lengths(code_lengths: &[u8]) -> Self {
        // Define an array to hold the amount of times a code length appears.
        // The index is the code length, and the value at the index is the
        // number of occurances.
        let mut occurances = [0u32; 256];

        // Get the higest code length in the array.
        let max_length = *code_lengths.iter().max().unwrap_or(&0) as usize;

        // Iterates over code_lengths, taking occurances in as acc, and taking
        // the current iterated value as idx. Then, acc is dereferenced to
        // directly modify occurances, and it is indexed by idx (the code
        // length) before being incremented while preventing overflow by
        // saturating_add. acc is then returned, and the fold operation repeats
        // until all members of code_lengths have been iterated over.
        code_lengths.iter().fold(&mut occurances, |acc, &idx| {
            (*acc)[idx as usize] = (*acc)[idx as usize].saturating_add(1);
            acc
        });

        // Intialize next_code and code as zeroes.
        let mut next_code = vec![0; max_length + 1];
        let mut code = 0;
        occurances[0] = 0;

        for i in 1..=max_length {
            code = (code + occurances[i - 1]) << 1;
            next_code[i] = code;
        }

        let mut codes = vec![None; code_lengths.len()];

        for j in 0..code_lengths.len() {
            let len = code_lengths[j] as usize;
            if len != 0 {
                codes[j] = Some(next_code[len]);
                next_code[len] += 1;
            }
        }

        let mut leaves = Vec::with_capacity(code_lengths.len());

        let mut nodes_left = BinaryHeap::new();
        let mut nodes_right = BinaryHeap::new();

        for (symbol, code) in codes.iter().enumerate() {
            if let Some(code) = code {
                let node = Node {
                    value: Some(symbol),
                    significance: *code as u64,
                    code: Code::from(*code, code_lengths[symbol]),
                    left: None,
                    right: None,
                };

                leaves.push(node.clone());
                match code.bit_index(code_lengths[symbol] - 1) {
                    0 => nodes_left.push(Box::new(node.clone())),
                    1 => nodes_right.push(Box::new(node.clone())),
                    _ => {}
                }
            }
        }

        let root = Node {
            value: None,
            significance: 0,
            code: Code::new(),
            left: Some(collect_from_heap(&mut nodes_left)),
            right: Some(collect_from_heap(&mut nodes_right)),
        };

        Self {
            root: root.clone(),
            leaves,
            current: Box::new(root),
        }
    }
    pub fn walk(&mut self, direction: u8) -> Option<usize> {
        let normalized_direction: u8 = match direction {
            0 => 0,
            1 => 1,
            _ => {
                eprintln!(
                    "Warning: Non-binary value given to binary function, value normalized to 1."
                );
                1
            }
        };

        match normalized_direction {
            0 => {
                if let Some(v) = self.current.left.clone() {
                    self.current = v.clone();
                    if let Some(value) = v.value {
                        self.current = Box::new(self.root.clone());
                        return Some(value);
                    } else {
                        return None;
                    }
                }
            }
            1 => {
                if let Some(v) = self.current.right.clone() {
                    self.current = v.clone();
                    if let Some(value) = v.value {
                        self.current = Box::new(self.root.clone());
                        return Some(value);
                    } else {
                        return None;
                    }
                }
            }
            _ => {}
        }
        None
    }
}

fn collect_from_heap(heap: &mut BinaryHeap<Box<Node>>) -> Box<Node> {
    while heap.len() > 1 {
        let node_1 = heap.pop().unwrap();
        let node_2 = heap.pop().unwrap();

        let parent_code = node_1.code.buffer >> 1;
        let parent_len = match node_1.code.length {
            v if v != 0 => v - 1,
            _ => 0,
        };

        let parent = Node {
            value: None,
            significance: parent_code as u64,
            code: Code::from(parent_code, parent_len),
            left: Some(node_2),
            right: Some(node_1),
        };

        heap.push(Box::new(parent));
    }

    heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::bitstream::BitStream;
    use crate::prefix::{Code, PrefixTree};

    use super::FIXED_CODE_LENGTHS;

    #[test]
    fn test_code() {
        // Create empty code.
        let mut new_code = Code::new();

        // Push the bits 1 and 0.
        new_code.push_bit(1);
        new_code.push_bit(0);

        // Create another code from new_code.
        let mut from_code = Code::from(new_code.buffer, new_code.length);

        // Push longer value.
        let new_value: u32 = 0b1111_0000;
        from_code.push(new_value, 8);

        // Check codes.
        assert_eq!(from_code.buffer, 0b00000000_00000000_00000010_11110000);
        assert_eq!(new_code.buffer, 0b00000000_00000000_00000000_00000010);

        // Check lengths.
        assert_eq!(from_code.length, 10);
        assert_eq!(new_code.length, 2);
    }
    #[test]
    fn test_prefix() {
        let code_lengths = [3, 3, 3, 3, 3, 2, 4, 4];

        let mut prefix_tree = PrefixTree::from_lengths(&code_lengths);

        let bits: u32 = 0b0100111001011100011101111;

        let mut stream = BitStream::new();

        stream.push(bits, 25);

        let mut results = Vec::new();

        for bit in stream {
            if let Some(v) = prefix_tree.walk(bit) {
                results.push(v);
            }
        }

        assert_eq!(results, vec![0, 1, 2, 3, 4, 5, 6, 7])
    }
}
