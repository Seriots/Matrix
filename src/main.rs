
mod utils;
mod core;
mod test;

use crate::core::*;

fn main() {
    println!("Please run `cargo test`");

    let u = Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.)]);

    println!("u = {}", u);
    println!("u = {}", u.norm());
}


