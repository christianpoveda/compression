pub trait Code<T, V> {
    fn transform(&self, symbol: &T) -> Option<&V>;
}
