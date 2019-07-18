extern crate shannon;

use shannon::Code;

#[test]
fn foo() {
    let freqs = vec![15usize, 7, 6, 6, 5];
    let code = Code::new(&freqs);
    for i in 0..5 {
        println!("{:?}", code.get(i));
    }
}
