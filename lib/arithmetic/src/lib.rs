use std::collections::HashMap;
use std::hash::Hash;

pub struct ArithmeticCode<T> {
    map: HashMap<T, Interval>,
}
impl<T> ArithmeticCode<T>
where
    T: Clone + Hash + Eq + std::fmt::Debug,
{
    pub fn recover(&self, data: u64) -> Option<Vec<T>> {
        println!("looking for: {:?}", data);

        let mut base = Interval::new(0, std::u64::MAX);
        let mut buffer = Vec::new();
        let mut go = true;
        while go {
            for (symbol, interval) in &self.map {
                let interval: Interval = base.contained(*interval)?;
                go = interval.len() > 1;
                println!("checking {:?}", interval);
                if interval.contains(data) {
                    buffer.push(symbol.clone());
                    base = interval;
                    println!("found");
                    break;
                }
            }
        }
        Some(buffer)
    }

    pub fn transform(&self, data: &[T]) -> Option<u64> {
        let mut data_iter = data.iter();
        let mut interval = *self.map.get(data_iter.next()?)?;
        for symbol in data_iter {
            interval = interval.contained(*self.map.get(symbol)?)?;
        }
        Some(interval.top)
    }

    pub fn from_data(data: &[T]) -> Self {
        let counts = Self::count_symbols(data);
        let total = data.len() as u64;

        let mut map = HashMap::new();
        let mut acc = 0;

        for (symbol, count) in counts {
            let length = ((std::u64::MAX as u128 * count as u128) / total as u128) as u64;
            map.insert(symbol, Interval::new(acc, acc + length));
            acc += length;
        }
        ArithmeticCode { map }
    }
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
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    bot: u64,
    top: u64,
}

impl Interval {
    fn new(bot: u64, top: u64) -> Self {
        Interval { bot, top }
    }

    fn len(&self) -> u64 {
        self.top - self.bot
    }

    fn contained(self, other: Self) -> Option<Self> {
        let len = self.len();
        Some(Self::new(
            self.bot + ((other.bot as u128 * self.len() as u128) / std::u64::MAX as u128) as u64,
            self.bot + ((other.top as u128 * self.len() as u128) / std::u64::MAX as u128) as u64,
        ))
    }

    fn contains(&self, value: u64) -> bool {
        self.top > value && self.bot <= value
    }
}
