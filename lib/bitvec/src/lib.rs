use std::convert::TryInto;
use std::fmt;
use std::u8::MAX;

const SIZE: u8 = 8;
const MAX_OFF: u8 = SIZE - 1;

#[derive(Clone, PartialEq, Eq)]
pub struct BitVec {
    vec: Vec<u8>,
    offset: u8,
}

impl fmt::Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;

        for byte in &self.vec[..self.vec.len().saturating_sub(1)] {
            write!(f, "{:08b} ", byte)?;
        }

        if let Some(&byte) = self.vec.last() {
            let string = &format!("{:08b}", byte)[..(self.offset + 1) as usize];
            write!(f, "{} ", string)?;
        }
        write!(f, "]")
    }
}

impl BitVec {
    pub fn new() -> Self {
        BitVec {
            vec: vec![],
            offset: 7,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vec.len() * 8 + self.offset as usize
    }

    pub fn push(&mut self, bit: bool) {
        let byte = if bit { 1u8.reverse_bits() } else { 0 };
        if self.offset == MAX_OFF {
            self.offset = 0;
            self.vec.push(byte);
        } else {
            self.offset += 1;
            *self.vec.last_mut().unwrap() |= byte >> self.offset;
        }
    }

    pub fn pop(&mut self) -> Option<bool> {
        let head = self.vec.last_mut()?;
        let mask = 2u8.pow((MAX_OFF - self.offset).into());

        let result = if (*head & mask) != 0 {
            *head &= MAX - mask;
            Some(true)
        } else {
            Some(false)
        };

        if self.offset == 0 {
            self.vec.pop().unwrap();
            if !self.is_empty() {
                self.offset = MAX_OFF;
            }
        } else {
            self.offset -= 1;
        }

        result
    }

    pub fn concat(&mut self, mut other: Self) {
        if self.offset == MAX_OFF {
            self.offset = other.offset;
            self.vec.extend(other.vec);
            return;
        }

        for byte in other.vec.drain(..) {
            let k = (other.offset + 1).min(MAX_OFF - self.offset);

            let mask = MAX - 2u8.pow((SIZE - k).into()) + 1;

            println!("{:b} {:b}", mask, byte);

            let new_byte = (byte & mask).wrapping_shr((SIZE - k).into());
            let rem_byte = (byte & (MAX - mask)).wrapping_shl(k.into());

            if let Some(head) = self.vec.last_mut() {
                *head += new_byte;
            } else {
                self.vec.push(new_byte);
            }

            self.offset = (self.offset + other.offset + 1) % SIZE;

            if self.offset != MAX_OFF {
                self.vec.push(rem_byte);
            }
        }
    }
}
