mod matrix;
mod vector;
mod test;

use matrix::Matrix;
use vector::Vector;


fn vector_ex00() {
    let mut u = Vector::from(vec![2., 3.]);
    let v = Vector::from(vec![5., 7.]);
    u.add(&v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from(vec![2., 3.]);
    let v = Vector::from(vec![5., 7.]);
    u.sub(&v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from(vec![2., 3.]);
    u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]
}

fn matrix_ex00() {
    let mut u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.]
        ]);
        let v = Matrix::from(vec![
        vec![7., 4.],
        vec![-2., 2.]
        ]);
        u.add(&v);
        println!("{}", u);
        // [8.0, 6.0]
        // [1.0, 6.0]
        let mut u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.]
        ]);
        let v = Matrix::from(vec![
        vec![7., 4.],
        vec![-2., 2.]
        ]);
        u.sub(&v);
        println!("{}", u);
        // [-6.0, -2.0]
        // [5.0, 2.0]
        let mut u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.]
        ]);
        u.scl(2.);
        println!("{}", u);
        // [2.0, 4.0]
        // [6.0, 8.0]
}

fn main() {
    vector_ex00();
    matrix_ex00();
    //println!("{}", matrix);
    //println!("matrix1: shape ={:?}, is_square={:?}", matrix.shape(), matrix.is_square());

    //let vector = Vector::from(vec!([25.0, 5.5884, 12.0, 98412.0]));
    //println!("{}", vector);
    //println!("{}", vector.size());
}
