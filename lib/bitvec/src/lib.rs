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
            // The offset is maximal to be consistent with non empty aligned vectors
            offset: MAX_OFF,
        }
    }

    pub fn is_empty(&self) -> bool {
        // Hopefully, the offset should be maximal
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        // Length in bits
        self.vec.len() * 8 + self.offset as usize
    }

    pub fn is_aligned(&self) -> bool {
        // Every aligned vector should use all its bits in its last byte, therefore its offset
        // should be maximal
        self.offset == MAX_OFF
    }

    pub fn push(&mut self, bit: bool) {
        // This is the byte to be pushed to the end of the vector. It is initialized with a 1 in
        // the greater digit if `bit` is true, it is zero otherwise.
        let byte = if bit { 1u8.reverse_bits() } else { 0 };
        if self.is_aligned() {
            // If the vector is aligned we can just push `byte` into the inner vector and set the
            // offset to 0
            self.offset = 0;
            self.vec.push(byte);
        } else {
            // When the vector is misaligned we increase the offset, shift `byte` to begin in the
            // offset and finally do the logical or operation between the shifted `byte` and the
            // last byte of the vector (this last part should be a noop if `byte` is zero)
            self.offset += 1;
            *self.vec.last_mut().unwrap() |= byte >> self.offset;
        }
    }

    pub fn pop(&mut self) -> Option<bool> {
        let head = self.vec.last_mut()?;
        // This mask contains a 1 in the exact position of the bit to be removed
        let mask = 2u8.pow((MAX_OFF - self.offset).into());

        let result = if (*head & mask) != 0 {
            // If the to-be-popped bit is 1, we use the mask to remove it
            *head &= MAX - mask;
            // Return `true` because the bit was 1
            Some(true)
        } else {
            // The bit was 0 and there is no need to change the last byte. Just return `false`
            Some(false)
        };

        if self.offset == 0 {
            // If the vector was using only one bit at the end, we pop the whole byte. This cannot
            // fail because an empty vector should be aligned.
            self.vec.pop().unwrap();
            // Align the vector
            self.offset = MAX_OFF;
        } else {
            // Otherwise we just decrease the offset
            self.offset -= 1;
        }

        result
    }

    pub fn concat(&mut self, mut other: Self) {
        if self.is_aligned() {
            // If the vector is aligned, we extend the inner vector with the contents of the other
            // vector and update the offset
            self.vec.extend(other.vec);
            self.offset = other.offset;
        } else if let Some(byte) = other.vec.pop() {
            // If the vector is not aligned and the other vector is not empty, we need to split
            // each byte of the other vector by the current vector's offset and add it to the
            // current vector in a two step process.

            // This is the number of unused bits in the last byte of the vector
            let k = MAX_OFF - self.offset;
            // This mask is used to extract the first `k` bits of each of the other vector
            // bytes
            let mask = MAX - 2u8.pow((SIZE - k).into()) + 1;
            // This mask is used to extract the last `SIZE - k` bits of each one of the other vector
            // bytes
            let inv_mask = MAX - mask;

            for byte in other.vec.drain(..) {
                // These are the first `k` bits of `byte` shifted to match with the last byte of
                // the current vector.
                let fst = (byte & mask).wrapping_shr((self.offset + 1).into());
                // These are the last `SIZE - k` bits of `byte` shifted to be aligned
                let lst = (byte & inv_mask).wrapping_shl(k.into());
                // The current vector cannot be empty because it is misaligned. So we add the `fst`
                // bits to the last byte
                *self.vec.last_mut().unwrap() += fst;
                // Finally we push the remaining bits in `lst`. The offset does not change given
                // that we are adding everything in blocks of SIZE.
                self.vec.push(lst);
            }

            // Now we need to add the last byte of the other vector it is special because it has
            // its own offset

            // This is the number of bits in `byte` that we can fit in the last byte of the current
            // vector, the rest of the logic is almost the same as before
            let k = (other.offset + 1).min(MAX_OFF - self.offset);
            let mask = MAX - 2u8.pow((SIZE - k).into()) + 1;
            let inv_mask = MAX - mask;
            let fst = (byte & mask).wrapping_shr((self.offset + 1).into());
            let lst = (byte & inv_mask).wrapping_shl(k.into());
            *self.vec.last_mut().unwrap() += fst;

            if k <= other.offset {
                // If there are remaining bits in `lst` we push them
                self.vec.push(lst);
                // The offset needs to be updated to include the number of bits in `lst`
                self.offset = other.offset - k;
            } else {
                // Otherwise we increase the offset to include the number of bits in `fst`
                self.offset += k;
            }
        }
    }
}
