use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use bitvec::BitVec;

pub struct ArithmeticCode<T> {
    map: HashMap<T, Interval>,
    total: u32,
}

impl<T: Debug + Clone + Eq + Hash> ArithmeticCode<T> {
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

    pub fn from_data(data: &[T]) -> Self {
        let mut map = HashMap::new();
        let counts = Self::count_symbols(data);
        let mut low_count = 0;
        for (symbol, count) in counts {
            let high_count = low_count + count as u32;
            map.insert(symbol, Interval::new(low_count, high_count));
            low_count = high_count;
        }

        ArithmeticCode {
            map,
            total: low_count,
        }
    }

    pub fn transform(&self, data: &[T]) -> Option<u32> {
        let mut int = Interval::new(0, std::u32::MAX.wrapping_shr(1));

        for symbol in data {
            let Interval {
                low: low_count,
                high: high_count,
            } = self.map.get(symbol)?;
            if int.high <= int.low {
                return None;
            }
            let step = (int.high - int.low + 1) / self.total;
            int.high = int.low + step * high_count - 1;
            int.low += step * low_count;
        }

        Some((int.high + int.low) / 2)
    }

    pub fn recover(&self, buffer: u32, len: usize) -> Option<Vec<T>> {
        let mut result = Vec::new();
        let mut int = Interval::new(0, std::u32::MAX.wrapping_shr(1));
        for _ in 0..len {
            let step = (int.high - int.low - 1) / self.total;
            let value = (buffer - int.low) / step;
            for (symbol, count) in &self.map {
                if value < count.high && value >= count.low {
                    result.push(symbol.clone());
                    int.high = int.low + step * count.high - 1;
                    int.low += step * count.low;
                    break;
                }
            }
        }
        Some(result)
    }
}

#[derive(Debug, Clone)]
struct Interval {
    low: u32,
    high: u32,
}

impl Interval {
    fn new(low: u32, high: u32) -> Self {
        Interval { low, high }
    }
}
