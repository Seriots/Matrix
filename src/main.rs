
mod utils;
mod core;
mod test;

use crate::core::*;

fn main() {
    println!("Please run `cargo test`");

    let u = Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.)]);

    let a = Complex::new(1., 2.);
    let b = Complex::new(3., 4.);
    println!("u = {}", a * b);
}


