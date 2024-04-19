
mod utils;
mod core;
mod test;

use crate::core::*;

fn main() {
    println!("Please run `cargo test`");

    let mat = Matrix::from(&[
        &[8., 5., -2., 4., 28.],
        &[4., 2.5, 20., 4., -4.],
        &[7., 5., 1., 4., 17.],
        ]);
    println!("{}", mat.row_echelon());
    
}
