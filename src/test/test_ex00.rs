
#[cfg(test)]
#[test]
fn vector() {
    use crate::core::Vector;

    //println!("===============Vector Ex00===============");
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.add(&v);
    assert_eq!(u.vector, vec![7., 10.]);

    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.sub(&v);
    assert_eq!(u.vector, vec![-3., -4.]);

    let mut u = Vector::from(&[2., 3.]);
    u.scl(2.);
    assert_eq!(u.vector, vec![4., 6.]);
}

#[cfg(test)]
#[test]
fn matrix() {
    use crate::core::Matrix;

    //println!("===============Matrix Ex00===============");
    let mut u = Matrix::from(&[
            &[1., 2.],
            &[3., 4.]
        ]);
        let v = Matrix::from(&[
           &[7., 4.],
            &[-2., 2.]
        ]);
        u.add(&v);
        assert_eq!(u.matrix, vec![
            vec![8., 6.],
            vec![1., 6.]
        ]);
        let mut u = Matrix::from(&[
            &[1., 2.],
            &[3., 4.]
        ]);
        let v = Matrix::from(&[
            &[7., 4.],
            &[-2., 2.]
        ]);
        u.sub(&v);
        assert_eq!(u.matrix, vec![
            vec![-6., -2.],
            vec![5., 2.]
        ]);
        let mut u = Matrix::from(&[
            &[1., 2.],
            &[3., 4.]
        ]);
        u.scl(2.);
        assert_eq!(u.matrix, vec![
            vec![2., 4.],
            vec![6., 8.]
        ]);
        // [2.0, 4.0]
        // [6.0, 8.0]
}