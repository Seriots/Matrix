mod matrix;
mod vector;
mod fma;
mod test;


#[cfg(test)]
#[test]
fn vector_ex00() {
    use vector::Vector;
    
    println!("===============Vector Ex00===============");
    let mut u = Vector::from(vec![2., 3.]);
    let v = Vector::from(vec![5., 7.]);
    u.add(&v);
    assert_eq!(u.vector, vec![7., 10.]);

    let mut u = Vector::from(vec![2., 3.]);
    let v = Vector::from(vec![5., 7.]);
    u.sub(&v);
    assert_eq!(u.vector, vec![-3., -4.]);

    let mut u = Vector::from(vec![2., 3.]);
    u.scl(2.);
    assert_eq!(u.vector, vec![4., 6.]);
}

#[cfg(test)]
#[test]
fn matrix_ex00() {
    use matrix::Matrix;

    println!("===============Matrix Ex00===============");
    let mut u = Matrix::from(vec![
            vec![1., 2.],
            vec![3., 4.]
        ]);
        let v = Matrix::from(vec![
            vec![7., 4.],
            vec![-2., 2.]
        ]);
        u.add(&v);
        assert_eq!(u.matrix, vec![
            vec![8., 6.],
            vec![1., 6.]
        ]);
        let mut u = Matrix::from(vec![
            vec![1., 2.],
            vec![3., 4.]
        ]);
        let v = Matrix::from(vec![
            vec![7., 4.],
            vec![-2., 2.]
        ]);
        u.sub(&v);
        assert_eq!(u.matrix, vec![
            vec![-6., -2.],
            vec![5., 2.]
        ]);
        let mut u = Matrix::from(vec![
            vec![1., 2.],
            vec![3., 4.]
        ]);
        u.scl(2.);
        assert_eq!(u.matrix, vec![
            vec![2., 4.],
            vec![6., 8.]
        ]);
        // [2.0, 4.0]
        // [6.0, 8.0]
}

use vector::Vector;
use vector::linear_combination;

fn main() {
    println!("Please run `cargo test`");
    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);
    println!("{}", linear_combination::<f32>(&[e1, e2, e3], &[10., -2.,
    0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination::<f32>(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
    
}
