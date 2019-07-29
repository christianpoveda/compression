use bitvec::BitVec;

use Content::*;

pub struct HuffmanCoding {
    map: Vec<BitVec>,
}

impl HuffmanCoding {
    pub fn from_freqs(freqs: &[usize]) -> Self {
        let mut queue = Vec::new();

        for (ind, &freq) in freqs.into_iter().enumerate() {
            queue.push(Node {
                freq: -(freq as isize),
                content: Leaf { ind },
            });
        }

        while queue.len() > 1 {
            let left = Box::new(queue.pop().unwrap());
            let right = Box::new(queue.pop().unwrap());
            let freq = left.freq + right.freq;

            let node = Node {
                freq,
                content: Internal { left, right },
            };

            match queue.binary_search_by_key(&freq, |node| node.freq) {
                Ok(i) | Err(i) => queue.insert(i, node),
            };

        }

        let tree = &queue[0];

        HuffmanCoding {
            map: (0..freqs.len()).map(|i| tree.vec(i)).collect(),
        }
    }

    pub fn get(&self, pos: usize) -> Option<&BitVec> {
        self.map.get(pos)
    }
}

struct Node {
    freq: isize,
    content: Content,
}

enum Content {
    Leaf { ind: usize },
    Internal { left: Box<Node>, right: Box<Node> },
}
