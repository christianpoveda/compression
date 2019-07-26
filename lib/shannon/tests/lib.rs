extern crate shannon;

use shannon::Coding;

#[test]
fn foo() {
    let freqs = vec![15usize, 7, 6, 6, 5];
    let coding = Coding::new(&freqs);
    for i in 0..5 {
        println!("{:?}", coding.get(i));
    }
}
