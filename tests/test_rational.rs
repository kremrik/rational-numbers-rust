use rational_rust::{RationalNumber};


#[test]
fn fraction_reduces() {
    let r = rational_rust::rat!(2/4).unwrap();
    let gold = rational_rust::rat!(1/2).unwrap();
    assert_eq!(gold, r);
}


#[test]
fn zero_numerator() {
    let r = rational_rust::rat!(0/2).unwrap();
    let gold = rational_rust::rat!(0/1).unwrap();
    assert_eq!(gold, r);
}


#[test]
fn zero_denominator_errors() {
    let r = rational_rust::rat!(1/0);
    assert!(r.is_err());
}


#[test]
fn addition_with_reduce() {
    let r1 = rational_rust::rat!(1/4).unwrap();
    let r2 = rational_rust::rat!(1/4).unwrap();
    let gold = rational_rust::rat!(1/2).unwrap();
    let output = r1 + r2;
    assert_eq!(gold, output);
}


#[test]
fn subtraction_with_reduce() {
    let r1 = rational_rust::rat!(2/4).unwrap();
    let r2 = rational_rust::rat!(1/4).unwrap();
    let gold = rational_rust::rat!(1/4).unwrap();
    let output = r1 - r2;
    assert_eq!(gold, output);
}


#[test]
fn assortment_of_operations() {
    let r1 = rational_rust::rat!(1/2).unwrap();
    let r2 = rational_rust::rat!(1/4).unwrap();
    let r3 = r1 + r2;  // 3/4
    let r4 = rational_rust::rat!(1/5).unwrap();
    let output = r4 - r3;  // 4/20 - 15/20 = -11/20
    let gold = rational_rust::rat!(-11/20).unwrap();
    assert_eq!(gold, output);
}
