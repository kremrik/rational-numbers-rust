use rational_rust::{RationalNumber, rat};


fn main() {
    let rat1 = rat!(1/4).unwrap();
    let rat2 = rat!(1/4).unwrap();
    println!("{:?}", rat1+rat2);
    println!("{:?}", rat!(6/8).unwrap());
}
