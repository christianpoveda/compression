use std::marker::PhantomData;

use bitvec::BitVec;

pub trait Code<T, U> {
    fn transform(&self, symbol: &T) -> Option<&U>;
}

pub struct Compressor<T, U> {
    code: U,
    _t: PhantomData<T>,
}

impl<T, U: Code<T, BitVec>> Compressor<T, U> {
    pub fn from_code(code: U) -> Self {
        Compressor {
            code,
            _t: PhantomData,
        }
    }

    pub fn compress(&self, data: &[T]) -> Option<BitVec> {
        let mut buffer = BitVec::new();

        for symbol in data {
            let result = self.code.transform(symbol)?.clone();
            buffer.concat(result);
        }

        Some(buffer)
    }
}
