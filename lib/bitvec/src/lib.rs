use std::fmt;
use std::u8::MAX;

const BLOCK_SIZE: u8 = 8;
const MAX_OFFSET: u8 = BLOCK_SIZE - 1;
const LEADING_ONE: u8 = 1u8.rotate_right(1);

#[derive(Clone, PartialEq, Eq)]
pub struct BitVec {
    blocks: Vec<u8>,
    offset: u8,
}

impl fmt::Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;

        for byte in &self.blocks[..self.blocks.len().saturating_sub(1)] {
            write!(f, "{:08b} ", byte)?;
        }

        if let Some(&byte) = self.blocks.last() {
            let string = &format!("{:08b}", byte)[..(self.offset + 1) as usize];
            write!(f, "{} ", string)?;
        }
        write!(f, "]")
    }
}

impl BitVec {
    pub fn new() -> Self {
        BitVec {
            blocks: Vec::new(),
            // The offset is maximal to be consistent with non empty aligned vectors
            offset: MAX_OFFSET,
        }
    }

    pub fn is_empty(&self) -> bool {
        // Hopefully, the offset should be maximal
        self.blocks.is_empty()
    }

    pub fn len(&self) -> usize {
        // Length in bits
        self.blocks.len() * (BLOCK_SIZE as usize) - (MAX_OFFSET - self.offset) as usize
    }

    pub fn is_aligned(&self) -> bool {
        // Every aligned vector should use all its bits in its last byte, therefore its offset
        // should be maximal
        self.offset == MAX_OFFSET
    }

    pub fn push(&mut self, bit: bool) {
        if self.is_aligned() {
            self.offset = 0;
            if bit {
                self.blocks.push(LEADING_ONE);
            } else {
                self.blocks.push(0);
            }
        } else {
            self.offset += 1;
            if bit {
                let block = self.blocks.last_mut().unwrap();
                *block += LEADING_ONE >> self.offset;
            }
        }
    }

    pub fn pop(&mut self) -> Option<bool> {
        let block = self.blocks.last_mut()?;

        let mask = LEADING_ONE >> self.offset;

        let bit = if (*block & mask) != 0 {
            *block -= mask;
            true
        } else {
            false
        };

        if self.offset == 0 {
            self.blocks.pop().unwrap();
            self.offset = MAX_OFFSET;
        } else {
            self.offset -= 1;
        }

        Some(bit)
    }

    pub fn concat(&mut self, mut other: Self) {
        if self.is_aligned() {
            // If the vector is aligned, we extend the inner vector with the contents of the other
            // vector and update the offset
            self.blocks.extend(other.blocks);
            self.offset = other.offset;
        } else if let Some(byte) = other.blocks.pop() {
            // If the vector is not aligned and the other vector is not empty, we need to split
            // each byte of the other vector by the current vector's offset and add it to the
            // current vector in a two step process.

            // This is the number of unused bits in the last byte of the vector
            let k = MAX_OFFSET - self.offset;
            // This mask is used to extract the first `k` bits of each of the other vector
            // bytes
            let mask = MAX - 2u8.pow((BLOCK_SIZE - k).into()) + 1;
            // This mask is used to extract the last `SIZE - k` bits of each one of the other vector
            // bytes
            let inv_mask = MAX - mask;

            for byte in other.blocks.drain(..) {
                // These are the first `k` bits of `byte` shifted to match with the last byte of
                // the current vector.
                let fst = (byte & mask).wrapping_shr((self.offset + 1).into());
                // These are the last `SIZE - k` bits of `byte` shifted to be aligned
                let lst = (byte & inv_mask).wrapping_shl(k.into());
                // The current vector cannot be empty because it is misaligned. So we add the `fst`
                // bits to the last byte
                *self.blocks.last_mut().unwrap() += fst;
                // Finally we push the remaining bits in `lst`. The offset does not change given
                // that we are adding everything in blocks of SIZE.
                self.blocks.push(lst);
            }

            // Now we need to add the last byte of the other vector it is special because it has
            // its own offset

            // This is the number of bits in `byte` that we can fit in the last byte of the current
            // vector, the rest of the logic is almost the same as before
            let k = (other.offset + 1).min(MAX_OFFSET - self.offset);
            let mask = MAX - 2u8.pow((BLOCK_SIZE - k).into()) + 1;
            let inv_mask = MAX - mask;
            let fst = (byte & mask).wrapping_shr((self.offset + 1).into());
            let lst = (byte & inv_mask).wrapping_shl(k.into());
            *self.blocks.last_mut().unwrap() += fst;

            if k <= other.offset {
                // If there are remaining bits in `lst` we push them
                self.blocks.push(lst);
                // The offset needs to be updated to include the number of bits in `lst`
                self.offset = other.offset - k;
            } else {
                // Otherwise we increase the offset to include the number of bits in `fst`
                self.offset += k;
            }
        }
    }
}
