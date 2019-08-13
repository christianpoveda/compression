use std::collections::HashMap;
use std::hash::Hash;

use bitvec::BitVec;
use compress::Code;

use Content::*;

pub struct HuffmanCode<T> {
    map: HashMap<T, BitVec>,
}

impl<T> Code<T, BitVec> for HuffmanCode<T>
where
    T: Eq + Hash + Clone,
{
    fn encode(&self, symbols: &[T]) -> Option<BitVec> {
        let mut buffer = BitVec::new();
        for symbol in symbols {
            buffer.concat(self.map.get(symbol)?.clone());
        }
        Some(buffer)
    }

    fn from_data(data: &[T]) -> Self {
        let counts = Self::count_symbols(data);

        let (freqs, mut vec): (Vec<usize>, Vec<BitVec>) = counts
            .iter()
            .map(|(_, count)| (count, BitVec::new()))
            .unzip();

        Self::step(&mut vec, &Node::from_freqs(&freqs));

        HuffmanCode {
            map: counts
                .into_iter()
                .zip(vec.into_iter())
                .map(|((symbol, _), bits)| (symbol, bits))
                .collect(),
        }
    }
}

impl<T> HuffmanCode<T>
where
    T: Clone + Hash + Eq,
{
    fn count_symbols(data: &[T]) -> Vec<(T, usize)> {
        let mut counts: HashMap<T, usize> = HashMap::new();

        for symbol in data {
            match counts.get_mut(symbol) {
                Some(count) => *count += 1,
                None => {
                    counts.insert(symbol.clone(), 1);
                }
            }
        }

        let mut counts: Vec<(T, usize)> = counts.into_iter().collect();
        counts.sort_by_key(|x| -(x.1 as isize));
        counts
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
