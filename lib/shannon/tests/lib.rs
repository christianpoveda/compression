extern crate shannon;

use bitvec::BitVec;
use compress::Code;

use shannon::ShannonCode;

#[test]
fn frequency_test() {
    let data = b"AAAAAAAAAABBBBCCCCCCCDDEEE";
    let code = ShannonCode::from_data(data);

    assert_eq!(code.encode(b"A"), Some(BitVec::from_block(0b00000000, 0)));
    assert_eq!(code.encode(b"B"), Some(BitVec::from_block(0b11000000, 2)));
    assert_eq!(code.encode(b"C"), Some(BitVec::from_block(0b10000000, 1)));
    assert_eq!(code.encode(b"D"), Some(BitVec::from_block(0b11110000, 3)));
    assert_eq!(code.encode(b"E"), Some(BitVec::from_block(0b11100000, 3)));
}
