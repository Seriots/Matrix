
mod utils;
mod core;
mod test;

use core::*;

fn main() {
    println!("Please run `cargo test`");
    let u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
        ]);
        let v = Vector::from(&[4., 2.]);
        println!("{}", u.mul_vec(&v));
        // [4.]
        // [2.]
        let u = Matrix::from(&[
        &[11., 3.],
        &[7., 1.],
        &[8., 5.],
        ]);
        let v = Vector::from(&[5., 6.]);
        println!("{}", u.mul_vec(&v));
        let u = Matrix::from(&[
        &[2., 0.],
        &[0., 2.],
        ]);
        let v = Vector::from(&[4., 2.]);
        println!("{}", u.mul_vec(&v));
        // [8.]
        // [4.]
        let u = Matrix::from(&[
        &[2., -2.],
        &[-2., 2.],
        ]);
        let v = Vector::from(&[4., 2.]);
        println!("{}", u.mul_vec(&v));

        let u = Matrix::from(&[
            &[2, -2],
            &[-2, 2],
            ]);
            let v = Vector::from(&[4, 2]);
            println!("{}", u.mul_vec(&v));
        // [4.]
        // [-4.]

        let u = Matrix::from(&[
            &[1., 0.],
            &[0., 1.],
            ]);
            let v = Matrix::from(&[
            &[1., 0.],
            &[0., 1.],
            ]);
            println!("{}", u.mul_mat(&v));
            // [1., 0.]
            // [0., 1.]

            let u = Matrix::from(&[
                &[1., 0.],
                &[0., 1.],
                ]);
                let v = Matrix::from(&[
                &[2., 1.],
                &[4., 2.],
                ]);
                println!("{}", u.mul_mat(&v));
                // [2., 1.]
                // [4., 2.]
                let u = Matrix::from(&[
                &[3., -5.],
                &[6., 8.],
                ]);
                let v = Matrix::from(&[
                &[2., 1.],
                &[4., 2.],
                ]);
                println!("{}", u.mul_mat(&v));
                // [-14., -7.]
                // [44., 22.]
            let u = Matrix::from(&[
            &[1., 0.],
            &[0., 1.],
            &[0., 1.],
            ]);
            let v = Matrix::from(&[
            &[2., 1., 5., 7.],
            &[4., 2., 8., 6.],
            ]);
            println!("{}", u.mul_mat(&v));
            // [2., 1.]
            // [4., 2.]
            //let u = Matrix::from(&[
            //    &[1., 0., 2.],
            //    &[0., 1., 2.],
            //    &[0., 1., 2.],
            //    ]);
            //    let v = Matrix::from(&[
            //    &[2., 1., 5., 7.],
            //    &[4., 2., 8., 6.],
            //    ]);
            //    println!("{}", u.mul_mat(&v));
            //    // [2., 1.]
            //    // [4., 2.]
}
