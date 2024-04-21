
mod utils;
mod core;
mod test;

use crate::core::*;

fn main() {
    println!("Please run `cargo test`");
    
    let u = Matrix::from(&[
        &[4., 1.],
        &[3., 1.],
    ]);
    println!("row echelon \n{}\n, det = {}", u.row_echelon(), u.determinant());
}
