extern crate huffman;

use compress::Code;
use huffman::HuffmanCode;

#[test]
fn foo() {
    let data = b"A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    // let data = b"AAAAAAAAAAAAAAABBBBBBBCCCCCCDDDDDDEEEEE";
    let code = HuffmanCode::from_data(data);
    for s in b"_ABCDE" {
        println!("{} -> {:?}", *s as char, code.transform(s));
    }
}
