use std::collections::HashMap;
use std::hash::Hash;

pub fn count_symbols<T: Hash + Eq + Clone>(data: &[T]) -> impl Iterator<Item = (T, usize)> {
    let mut counts: HashMap<T, usize> = HashMap::new();

    for symbol in data {
        match counts.get_mut(symbol) {
            Some(count) => *count += 1,
            None => {
                counts.insert(symbol.clone(), 1);
            }
        }
    }

    counts.into_iter()
}
