extern crate arithmetic;

use arithmetic::ArithmeticCode;

#[test]
fn foo() {
    let data = b"A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    let msg = b"A_DEAD_DAD_";
    let code = ArithmeticCode::from_data(data);
    let comp = code.transform(msg).unwrap();
    println!("{:?} -> {:0x}", (String::from_utf8(msg.to_vec())), comp);
    println!(
        "{:?}",
        String::from_utf8(code.recover(comp, msg.len()).unwrap())
    );
}
