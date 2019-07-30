extern crate shannon;

use compress::Code;
use shannon::ShannonCode;

#[test]
fn foo() {
    let data = b"AAAAAAAAAAAAAAABBBBBBBCCCCCCDDDDDDEEEEE";
    let code = ShannonCode::from_data(data);
    for s in b"ABCDE" {
        println!("{} -> {:?}", *s as char, code.transform(s));
    }
}
