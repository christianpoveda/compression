use std::collections::HashMap;
use std::hash::Hash;

use bitvec::BitVec;
use compress::Code;

pub struct ShannonCode<T> {
    map: HashMap<T, BitVec>,
}

impl<T: Eq + Hash + Clone> Code<T, BitVec> for ShannonCode<T> {
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

        Self::step(&mut vec, &freqs);

        ShannonCode {
            map: counts
                .into_iter()
                .zip(vec.into_iter())
                .map(|((symbol, _), bits)| (symbol, bits))
                .collect(),
        }
    }
}

impl<T> ShannonCode<T>
where
    T: Hash + Clone + Eq,
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
        counts.sort_by_key(|x| x.1);
        counts
    }

    fn step(vec: &mut [BitVec], freqs: &[usize]) {
        if vec.len() <= 1 {
            return;
        }

        let index = Self::find_middle(freqs) + 1;

        let (lower_vec, upper_vec) = vec.split_at_mut(index);
        let (lower_fqs, upper_fqs) = freqs.split_at(index);

        for bits in lower_vec.iter_mut() {
            bits.push(true);
        }

        for bits in upper_vec.iter_mut() {
            bits.push(false);
        }

        Self::step(lower_vec, lower_fqs);
        Self::step(upper_vec, upper_fqs);
    }

    fn find_middle(freqs: &[usize]) -> usize {
        let mut top = 0;
        let mut bot = freqs.iter().sum::<usize>();

        freqs
            .iter()
            .map(|i| {
                top += i;
                bot -= i;
                if top > bot {
                    top - bot
                } else {
                    bot - top
                }
            })
            .enumerate()
            .min_by_key(|(_, d)| *d)
            .unwrap()
            .0
    }
}

