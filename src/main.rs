
mod utils;
mod core;
mod test;

use crate::core::*;

fn main() {
    println!("Please run `cargo test`");

    let u = projection(80., 1., 0.1, 100.);

    println!("{}", u);
    save_matrix(&u, "proj");
}


