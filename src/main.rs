
mod utils;
mod core;
mod test;

use core::*;

fn main() {
    println!("Please run `cargo test`");

    let u = Matrix::from(&[
        &[1., 0., 0.],
        &[2., 3., 4.],
    ]);
    let v = u.transpose();
    println!("{}", v);

    let u = Matrix::from(&[
        &[1., 2.],
    ]);
    let v = u.transpose();
    println!("{}", v);

    let u = Matrix::from(&[
        &[1.],
        &[2.],
    ]);
    let v = u.transpose();
    println!("{}", v);

    let u = Matrix::from(&[
        &[1., 2.],
        &[1., 2.],
        &[1., 2.],
        &[1., 2.],
        &[1., 2.],
    ]);
    let v = u.transpose();
    println!("{}", v);

    let u = Matrix::from(&[
        &[1., 2., 3., 4., 5.],
        &[1., 2., 3., 4., 5.],
        &[1., 2., 3., 4., 5.],
        &[1., 2., 3., 4., 5.],
        &[1., 2., 3., 4., 5.],
    ]);
    let v = u.transpose();
    println!("{}", v);
    
}
