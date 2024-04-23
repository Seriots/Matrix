
mod utils;
mod core;
mod test;

use crate::core::*;

fn main() {
    println!("Please run `cargo test`");

    let proj_mat = projection(90., 1., 0.01, 20.);
    save_matrix(&proj_mat, "proj");
}


