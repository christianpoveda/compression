extern crate bitvec;

use bitvec::BitVec;
#[test]
fn debug() {
    let mut a = BitVec::new();

    assert_eq!("[ ]", format!("{:?}", a));

    a.push(true);
    a.push(false);
    a.push(true);
    a.push(false);
    a.push(true);

    assert_eq!("[ 10101 ]", format!("{:?}", a));

    a.push(true);
    a.push(true);
    a.push(true);
    a.push(true);

    assert_eq!("[ 10101111 1 ]", format!("{:?}", a));
}

#[test]
fn push() {
    let mut bec = BitVec::new();
    let mut vec = Vec::new();

    bec.push(true);
    vec.push(true);

    assert_eq!(bec.pop(), vec.pop());
}

#[test]
fn concat() {
    // a = 10101
    let mut a = BitVec::new();
    a.push(true);
    a.push(false);
    a.push(true);
    a.push(false);
    a.push(true);

    // c = 1 11110101
    let mut c = a.clone();
    c.push(true);
    c.push(true);
    c.push(true);
    c.push(true);

    // b = 1111
    let mut b = BitVec::new();
    b.push(true);
    b.push(true);
    b.push(true);
    b.push(true);

    // a = 1 11110101
    a.concat(b);
    assert_eq!(a, c);
}

#[test]
fn concat_complete() {
    // a = 0101
    let mut a = BitVec::new();
    a.push(true);
    a.push(false);
    a.push(true);
    a.push(false);

    // c = 11110101
    let mut c = a.clone();
    c.push(true);
    c.push(true);
    c.push(true);
    c.push(true);

    // b = 1111
    let mut b = BitVec::new();
    b.push(true);
    b.push(true);
    b.push(true);
    b.push(true);

    // a = 11110101
    a.concat(b);
    assert_eq!(a, c);
}

#[test]
fn concat_from_empty() {
    let mut a = BitVec::new();
    let c = a.clone();
    let b = BitVec::new();
    a.concat(b);
    assert_eq!(a, c);
}

#[test]
fn concat_to_empty() {
    let mut a = BitVec::new();
    let mut b = BitVec::new();

    b.push(true);
    b.push(false);
    b.push(true);
    b.push(false);
    b.push(true);

    let c = b.clone();
    a.concat(b);
    assert_eq!(a, c);
}
