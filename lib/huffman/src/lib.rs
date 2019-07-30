use bitvec::BitVec;

use Content::*;

pub struct HuffmanCoding {
    map: Vec<BitVec>,
}

impl HuffmanCoding {
    pub fn from_freqs(freqs: &[usize]) -> Self {
        let tree = Node::from_freqs(freqs);
        let mut map = vec![BitVec::new(); freqs.len()];
        Self::step(&mut map, &tree);
        HuffmanCoding { map }
    }

    fn step(map: &mut [BitVec], tree: &Node) {
        match &tree.content {
            Internal(left, right) => {
                left.push_bit(false, map);
                right.push_bit(true, map);

                Self::step(map, &left);
                Self::step(map, &right);
            }
            Leaf(_) => {}
        }
    }

    pub fn get(&self, pos: usize) -> Option<&BitVec> {
        self.map.get(pos)
    }
}

enum Content {
    Leaf(usize),
    Internal(Box<Node>, Box<Node>),
}

struct Node {
    freq: isize,
    content: Content,
}

impl Node {
    fn from_freqs(freqs: &[usize]) -> Self {
        let mut queue = Vec::new();

        for (ind, &freq) in freqs.into_iter().enumerate() {
            queue.push(Node {
                freq: -(freq as isize),
                content: Leaf(ind),
            });
        }

        while queue.len() > 1 {
            let left = Box::new(queue.pop().unwrap());
            let right = Box::new(queue.pop().unwrap());
            let freq = left.freq + right.freq;

            let node = Node {
                freq,
                content: Internal(left, right),
            };

            match queue.binary_search_by_key(&freq, |node| node.freq) {
                Ok(i) | Err(i) => {
                    queue.insert(i, node);
                }
            };
        }

        queue.pop().unwrap()
    }

    fn push_bit(&self, bit: bool, vecs: &mut [BitVec]) {
        match &self.content {
            Leaf(ind) => vecs[*ind].push(bit),
            Internal(left, right) => {
                left.push_bit(bit, vecs);
                right.push_bit(bit, vecs);
            }
        }
    }
}
