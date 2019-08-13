pub mod utils;

pub trait Code<T, U> {
    fn from_data(data: &[T]) -> Self;
    fn encode(&self, symbols: &[T]) -> Option<U>;
}
