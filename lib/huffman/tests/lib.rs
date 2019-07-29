extern crate huffman;

use huffman::HuffmanCoding;

#[test]
fn foo() {
    let freqs = vec![40usize, 35, 20, 5];
    let coding = HuffmanCoding::from_freqs(&freqs);
    for i in 0..4 {
        println!("{:?}", coding.get(i));
    }
}
