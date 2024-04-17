
mod utils;
mod core;
mod test;

use core::*;

fn main() {
    println!("Please run `cargo test`");
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![ 1., -1.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(vec![1 as usize, 2, 3]);
    let v = Vector::from(vec![4, 5, 6]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 0.974631846

}
