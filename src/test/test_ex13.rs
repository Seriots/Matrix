
#[cfg(test)]
#[test]
fn test_rank() {
    use crate::Matrix;

    let u = Matrix::from(&[
        &[1., 0., 0.],
        &[0., 1., 0.],
        &[0., 0., 1.],
    ]);
    assert_eq!(u.rank(), 3);
    // 3
    
    let u = Matrix::from(&[
        &[ 1., 2., 0., 0.],
        &[ 2., 4., 0., 0.],
        &[-1., 2., 1., 1.],
    ]);
    assert_eq!(u.rank(), 2);
    // 2
    
    let u = Matrix::from(&[
        &[ 8., 5., -2.],
        &[ 4., 7., 20.],
        &[ 7., 6., 1.],
        &[21., 18., 7.],
    ]);
    assert_eq!(u.rank(), 3);
    // 3
}

#[cfg(test)]
#[test]
fn test_rank_complex() {
    use crate::Matrix;
    use crate::Complex;

    let u = Matrix::from(&[
        &[Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        &[Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)],
        &[Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    assert_eq!(u.rank(), 3);

    let u = Matrix::from(&[
        &[Complex::new(1., 2.), Complex::new(2., 4.), Complex::new(0., 0.), Complex::new(0., 0.)],
        &[Complex::new(2., 4.), Complex::new(4., 8.), Complex::new(0., 0.), Complex::new(0., 0.)],
        &[Complex::new(-1., 2.), Complex::new(2., 4.), Complex::new(1., 2.), Complex::new(1., 2.)],
    ]);
    assert_eq!(u.rank(), 2);

    let u = Matrix::from(&[
        &[Complex::new(8., 2.), Complex::new(5., -3.), Complex::new(-2., 1.)],
        &[Complex::new(4., 1.), Complex::new(7., 1.), Complex::new(20., 1.)],
        &[Complex::new(7., 1.), Complex::new(6., 1.), Complex::new(1., 1.)],
        &[Complex::new(21., 1.), Complex::new(18., 1.), Complex::new(7., 1.)],
    ]);

    assert_eq!(u.rank(), 3);
    
}