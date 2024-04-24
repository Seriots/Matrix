
mod utils;
mod core;
mod test;

use crate::core::*;

fn main() {
    println!("This main is for the projection matrix, please run `cargo test` instead");

    let proj_mat = projection(90., 1., 0., 20.);
    save_matrix(&proj_mat, "proj");
}


