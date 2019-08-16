use std::fmt;

type Block = u8;
const BLOCK_SIZE: Block = 8;
const MAX_OFFSET: Block = BLOCK_SIZE - 1;
const LEADING_ONE: Block = (1 as Block).rotate_right(1);

#[derive(Clone, PartialEq, Eq)]
pub struct BitVec {
    blocks: Vec<Block>,
    offset: Block,
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

    pub fn from_block(block: Block, offset: Block) -> Self {
        BitVec {
            blocks: vec![block],
            offset,
        }
    }

    pub fn from_bytes(mut bytes: Vec<u8>) -> Self {
        loop {
            match bytes.last() {
                Some(block) => {
                    let zeros = block.trailing_zeros();
                    if zeros == 8 {
                        bytes.pop().unwrap();
                    } else {
                        return BitVec {
                            blocks: bytes,
                            offset: MAX_OFFSET - zeros as Block,
                        };
                    }
                }
                None => {
                    return Self::new();
                }
            }
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

            // This is the number of unused bits in the tail of `self`
            let limit = MAX_OFFSET - self.offset;

            for byte in other.blocks.drain(..) {
                // These are the first `limit` bits of `byte` aligned to start at
                // `BLOCK_SIZE - limit`
                let fst = byte.wrapping_shr((BLOCK_SIZE - limit).into());
                // These are the last `BLOCK_SIZE - limit` bits of `byte` aligned to start at `0`
                let lst = byte.wrapping_shl(limit.into());
                // The current vector cannot be empty because it is misaligned. So we add the `fst`
                // bits to the tail of `self`
                *self.blocks.last_mut().unwrap() += fst;
                // Finally we push the remaining bits in `lst`. The offset does not change given
                // that we are adding everything in blocks of SIZE.
                self.blocks.push(lst);
            }

            // Now we have to deal with the tail of `other`

            // We are going to add the first `limit` bits of the tail of `other` to the tail of
            // `self` in the same way as in the for loop above.
            let fst = byte.wrapping_shr((BLOCK_SIZE - limit).into());
            *self.blocks.last_mut().unwrap() += fst;

            // Now we need to decide what to do with the remaining bits in the tail of `other`.
            // Here we check if there are in fact any remaining bits in the tail of `other`.
            if other.offset + 1 <= limit {
                // This means that there are less used bits in the tail of `other` than unused bits
                // in the tail of `self`. Thus, we do not have any remaining bits to add to `self`.

                // We increase `self.offset` by the number of added bits, which is exactly the
                // number of used bits in the tail of `other`
                self.offset += other.offset + 1;
            } else {
                // This means that there are some remaining bits in the tail of `other`. We have to
                // extract them and push it to `self.blocks`
                let lst = byte.wrapping_shl(limit.into());
                self.blocks.push(lst);

                // Ue update `self` offset to be `other.offset` minus the number of bytes that we
                // already added from `fst`
                self.offset = other.offset - limit;
            }
        }
    }
}
